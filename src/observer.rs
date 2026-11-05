use crate::LabResult;
use serde_json::Value;
pub fn load(directory: &std::path::Path) -> LabResult<Value> {
    fn read(path: &std::path::Path) -> LabResult<Value> {
        serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())
    }
    let result = read(&directory.join("result.json"))?;
    let run = read(&directory.join("run.json"))?;
    if result["status"] != "functional_pass" {
        return Err("observer input is not functional_pass".into());
    }
    if run["case_sha256"].as_str() != Some(&crate::sha256_file(&directory.join("case.json"))?) {
        return Err("archived case hash mismatch".into());
    }
    if run.get("schema").is_some() || result.get("schema").is_some() {
        crate::trace_seal::verify_archived(directory, &run, &result)?;
    } else {
        let trace: crate::Trace = serde_json::from_value(read(&directory.join("trace.json"))?)
            .map_err(|e| e.to_string())?;
        crate::validate_trace(&trace)?;
    }
    Ok(
        serde_json::json!({"case":read(&directory.join("case.json"))?,"artifact_fingerprint":result["artifact_fingerprint"],"runner_sha256":run["runner_sha256"],"config":read(&directory.join("effective-config.json"))?,"observations":read(&directory.join("observations.json"))?,"persisted_machine":read(&directory.join("persisted-machine.json"))?}),
    )
}
pub fn check_pair(a: &Value, b: &Value) -> LabResult<Value> {
    if a.pointer("/case/observer_mode").and_then(Value::as_str) != Some("every_tick")
        || b.pointer("/case/observer_mode").and_then(Value::as_str) != Some("terminal")
    {
        return Err("observer pair requires every_tick then terminal".into());
    }
    let mut ac = a["case"].clone();
    let mut bc = b["case"].clone();
    ac.as_object_mut()
        .ok_or("invalid case")?
        .remove("observer_mode");
    bc.as_object_mut()
        .ok_or("invalid case")?
        .remove("observer_mode");
    if ac != bc {
        return Err("case drift".into());
    }
    for key in ["artifact_fingerprint", "runner_sha256", "config"] {
        if a.get(key).is_none() || a.get(key) != b.get(key) {
            return Err(format!("{key} drift"));
        }
    }
    let aa = a["observations"].as_array().ok_or("missing observations")?;
    let bb = b["observations"].as_array().ok_or("missing observations")?;
    if aa.is_empty() || aa.len() != bb.len() {
        return Err("observation count differs".into());
    }
    for (x, y) in aa.iter().zip(bb) {
        for key in ["kind", "sequence", "action", "state"] {
            if x.get(key).is_none() || x.get(key) != y.get(key) {
                return Err(format!("endpoint {key} mismatch"));
            }
        }
        let mut x = x.clone();
        let mut y = y.clone();
        for value in [&mut x, &mut y] {
            let object = value.as_object_mut().ok_or("observation not object")?;
            object.remove("measurement");
            object.remove("tick_samples");
        }
        if x != y {
            return Err("endpoint metadata mismatch".into());
        }
    }
    if a.get("persisted_machine").is_none()
        || a.get("persisted_machine") != b.get("persisted_machine")
    {
        return Err("persisted machine mismatch".into());
    }
    let persisted = crate::nbt::persisted_projection(&a["persisted_machine"])?;
    if persisted != aa.last().unwrap()["state"]["machine"]["nbt"] {
        return Err("persisted versus live mismatch".into());
    }
    let mut steps = 0;
    for (run, mode) in [(aa, "every_tick"), (bb, "terminal")] {
        let mut previous = None;
        for observation in run {
            let state = &observation["state"];
            let end = crate::lab::observation_tick(state)?;
            if observation["action"] == "step" {
                let start = previous.ok_or("step lacks initial state")?;
                let ticks = end.checked_sub(start).ok_or("tick regression")?;
                let samples: Vec<crate::capture::TickSample> =
                    serde_json::from_value(observation["tick_samples"].clone())
                        .map_err(|e| e.to_string())?;
                crate::capture::validate_batch(start, ticks, mode, &samples, state)?;
                crate::measurement::validate(&observation["measurement"], start, ticks, mode)?;
                steps += 1;
            }
            previous = Some(end);
        }
    }
    if steps == 0 {
        return Err("observer comparison requires actual step".into());
    }
    Ok(
        serde_json::json!({"observer_endpoint_equivalent":true,"independent_persistence":true,"step_batches_per_mode":steps/2,"scope":"every-tick versus terminal capture with the same bridge/controller","qualified":false,"optimization_proven":false,"port_equivalence_proven":false}),
    )
}

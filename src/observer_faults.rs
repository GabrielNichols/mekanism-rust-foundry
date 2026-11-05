use crate::{LabResult, capture, lab::atomic_json};
use serde_json::{Value, json};
use std::{fs, path::Path, time::Instant};

const START: u64 = 100;
const TICKS: u64 = 3;
const VALIDATIONS: usize = 200;

fn detect(reply: &Value, world: &Value) -> LabResult<()> {
    capture::validate_reply(START, TICKS, "every_tick", reply)?;

    // The scenario contract is independent of the synthetic observer and world.
    for (index, (tick, progress)) in [("101", 41), ("102", 42), ("103", 43)]
        .into_iter()
        .enumerate()
    {
        witness(&reply["tick_samples"][index]["state"], tick, progress)
            .map_err(|error| format!("observer sample {index}: {error}"))?;
    }
    witness(world, "103", 43).map_err(|error| format!("independent post-read world: {error}"))
}

fn witness(state: &Value, tick: &str, progress: u32) -> LabResult<()> {
    for (key, expected) in [
        ("tick", json!(tick)),
        ("progress", json!(progress)),
        ("ore", json!(1)),
    ] {
        if state[key] != expected {
            return Err(format!(
                "{key}: expected {expected}, observed {}",
                state[key]
            ));
        }
    }
    Ok(())
}

fn clean() -> (Value, Value) {
    let mut world = json!({"tick":START.to_string(),"progress":40,"ore":1});
    let mut samples = Vec::new();
    let mut timings = Vec::new();
    for tick in START + 1..=START + TICKS {
        world["tick"] = json!(tick.to_string());
        world["progress"] = json!(world["progress"].as_u64().unwrap() + 1);
        samples.push(json!({"tick":world["tick"],"phase":"end_server_tick",
            "owner_thread":true,"state":world.clone()}));
        timings.push(json!({"tick":world["tick"],"bridge_pre_to_post_ns":"10","snapshot_ns":"1"}));
    }
    (
        json!({"state":world.clone(),"tick_samples":samples,"measurement":{
            "batch_wall_ns":"50","process_cpu_ns":"30","gc_pause_ms":"0","gc_collections":"0",
            "heap_used_bytes":"100","ticks":timings
        }}),
        world,
    )
}

#[derive(Clone, Copy)]
enum Fault {
    None,
    Drop,
    Repeat,
    Phase,
    Thread,
    Stale,
    Endpoint,
    Timing,
    PostReadMutation,
}

impl Fault {
    fn apply(self, reply: &mut Value, world: &mut Value) {
        match self {
            Self::None => {}
            Self::Drop => {
                reply["tick_samples"].as_array_mut().unwrap().remove(1);
            }
            Self::Repeat => reply["tick_samples"][1] = reply["tick_samples"][0].clone(),
            Self::Phase => reply["tick_samples"][1]["phase"] = json!("before_server_tick"),
            Self::Thread => reply["tick_samples"][1]["owner_thread"] = json!(false),
            Self::Stale => {
                reply["tick_samples"][1]["state"]["progress"] =
                    reply["tick_samples"][0]["state"]["progress"].clone();
            }
            Self::Endpoint => reply["state"]["ore"] = json!(2),
            Self::Timing => reply["measurement"]["ticks"][1]["tick"] = json!("101"),
            Self::PostReadMutation => world["ore"] = json!(2),
        }
    }
}

const CASES: [(&str, Fault, Option<&str>); 9] = [
    ("clean", Fault::None, None),
    (
        "missing_sample",
        Fault::Drop,
        Some("capture sample count mismatch"),
    ),
    (
        "duplicate_tick",
        Fault::Repeat,
        Some("tick continuity mismatch"),
    ),
    ("wrong_phase", Fault::Phase, Some("capture phase mismatch")),
    (
        "wrong_thread",
        Fault::Thread,
        Some("capture thread is not owner"),
    ),
    (
        "stale_state_relabelled",
        Fault::Stale,
        Some("observer sample 1: progress"),
    ),
    (
        "forged_endpoint",
        Fault::Endpoint,
        Some("final state differs"),
    ),
    (
        "wrong_measurement_tick",
        Fault::Timing,
        Some("measurement tick mismatch"),
    ),
    (
        "inventory_changed_after_read",
        Fault::PostReadMutation,
        Some("independent post-read world: ore"),
    ),
];

pub fn run(output: &Path) -> LabResult<()> {
    fs::create_dir(output).map_err(|error| format!("output must be new: {error}"))?;
    let mut results = Vec::new();
    for (name, fault, expected) in CASES {
        let (mut reply, mut world) = clean();
        fault.apply(&mut reply, &mut world);
        let mut times = Vec::with_capacity(VALIDATIONS);
        let mut diagnostic = None;
        let mut matched = true;
        for _ in 0..VALIDATIONS {
            let start = Instant::now();
            let result = detect(&reply, &world);
            times.push(start.elapsed().as_nanos() as u64);
            diagnostic = result.err();
            matched &= expected.map_or(diagnostic.is_none(), |prefix| {
                diagnostic
                    .as_deref()
                    .is_some_and(|error| error.starts_with(prefix))
            });
        }
        times.sort_unstable();
        results.push(json!({
            "case":name,"expected_error_prefix":expected,"diagnostic":diagnostic,"matched":matched,
            "validator_wall_ns":{"samples":VALIDATIONS,"p50":times[VALIDATIONS/2-1],
                "p95":times[(VALIDATIONS*95).div_ceil(100)-1],"max":times[VALIDATIONS-1]},
            "synthetic_reply":reply,"independent_simulated_world_after_read":world,
        }));
    }
    let passed = results.iter().all(|row| row["matched"] == true);
    atomic_json(
        &output.join("result.json"),
        &json!({
            "schema":1,"scope":"simulated_world_and_observer","status":if passed {"pass"} else {"non_pass"},
            "qualified":false,"port_equivalence_proven":false,"optimization_proven":false,"cases":results,
        }),
    )?;
    if !passed {
        return Err("simulated observer campaign diverged from expected detections".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn accepts_unmodified_observations_and_independent_world() {
        let (reply, world) = clean();
        detect(&reply, &world).unwrap();
    }

    #[test]
    fn rejects_missing_and_misattributed_capture() {
        let (mut reply, world) = clean();
        reply["tick_samples"].as_array_mut().unwrap().remove(1);
        assert!(detect(&reply, &world).unwrap_err().contains("count"));
        let (mut reply, world) = clean();
        reply["tick_samples"][1]["owner_thread"] = json!(false);
        assert!(detect(&reply, &world).unwrap_err().contains("owner"));
    }

    #[test]
    fn rejects_relabelled_stale_payload_and_mutation_after_read() {
        let (mut reply, mut world) = clean();
        reply["tick_samples"][1]["state"]["progress"] = json!(41);
        assert!(detect(&reply, &world).unwrap_err().contains("progress"));
        let (reply, _) = clean();
        world["ore"] = json!(2);
        assert!(detect(&reply, &world).unwrap_err().contains("ore"));
    }

    #[test]
    fn rejects_wrong_tick_in_measurement() {
        let (mut reply, world) = clean();
        reply["measurement"]["ticks"][1]["tick"] = json!("101");
        assert!(
            detect(&reply, &world)
                .unwrap_err()
                .contains("measurement tick")
        );
    }
}

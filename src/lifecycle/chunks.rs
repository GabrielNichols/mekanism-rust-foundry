use super::{Environment, Observer, bounded_map, elapsed_ns, lab, launch, validate_advance};
use crate::LabResult;
use serde_json::{Value, json};
use std::{
    fs,
    io::{ErrorKind, Read, Seek, SeekFrom},
    path::Path,
    time::Instant,
};

const TARGET: i32 = 2000;

fn validate(
    before: &Value,
    after: &Value,
    loads: u64,
    persisted: bool,
    force_load: bool,
) -> LabResult<()> {
    if lab::observation_tick(before)? != lab::observation_tick(after)? {
        return Err("chunk observer changed the frozen clock".into());
    }
    if before["resident"] != false
        || before["holder"] != false
        || before["forced"] != false
        || after["forced"] != false
        || before["debug"].as_str().is_none()
        || after["debug"].as_str().is_none()
    {
        return Err("invalid unloaded chunk baseline or forced-chunk change".into());
    }
    if force_load {
        if after["resident"] != true
            || loads == 0
            || !persisted
            || after["debug"] == before["debug"]
        {
            return Err("load fault requires live residency, a load event, changed chunk state and a persisted record".into());
        }
    } else if before != after || loads != 0 || persisted {
        return Err("chunk observer violated neutrality".into());
    }
    Ok(())
}

fn validate_followup(
    immediate: &Value,
    later: &Value,
    loads: u64,
    force_load: bool,
) -> LabResult<()> {
    let tick = lab::observation_tick(immediate)?;
    if !validate_advance(tick, 3, lab::observation_tick(later)?)? {
        return Err("chunk witness window did not reach three ticks".into());
    }
    if !force_load
        && (loads != 0
            || ["resident", "holder", "forced", "debug"]
                .iter()
                .any(|field| immediate[*field] != later[*field]))
    {
        return Err("chunk observer violated delayed neutrality".into());
    }
    if later["forced"] != false {
        return Err("forced chunk state changed during the witness window".into());
    }
    Ok(())
}

fn region_has_target(dir: &Path) -> LabResult<bool> {
    let region = TARGET.div_euclid(32);
    let path = dir.join(format!("world/region/r.{region}.{region}.mca"));
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.to_string()),
    };
    let coord = TARGET.rem_euclid(32) as u64;
    file.seek(SeekFrom::Start((coord + coord * 32) * 4))
        .map_err(|error| error.to_string())?;
    let mut header = [0; 4];
    file.read_exact(&mut header)
        .map_err(|error| error.to_string())?;
    if header == [0; 4] {
        return Ok(false);
    }
    let sector = u64::from(u32::from_be_bytes([0, header[0], header[1], header[2]]));
    if sector < 2 || header[3] == 0 {
        return Err("invalid persisted target chunk allocation".into());
    }
    file.seek(SeekFrom::Start(sector * 4096))
        .map_err(|error| error.to_string())?;
    let mut record = [0; 5];
    file.read_exact(&mut record)
        .map_err(|error| error.to_string())?;
    let len = u64::from(u32::from_be_bytes(record[..4].try_into().unwrap()));
    if len < 2
        || len + 4 > u64::from(header[3]) * 4096
        || file.metadata().map_err(|error| error.to_string())?.len() < sector * 4096 + 4 + len
    {
        return Err("invalid persisted target chunk record".into());
    }
    Ok(true)
}

fn trial(environment: &Environment, output: &Path, force_load: bool) -> LabResult<Value> {
    fs::create_dir(output).map_err(|error| error.to_string())?;
    let dir = output.join("instance");
    fs::create_dir(&dir).map_err(|error| error.to_string())?;
    let mut server = launch(
        environment,
        &dir,
        output.join("server"),
        Observer::Chunk(force_load),
        100,
    )?;
    let start_tick = server.clock()?;
    let start = Instant::now();
    let reply = server
        .bridge
        .as_mut()
        .ok_or("chunk probe needs the bridge")?
        .request("chunk_probe", &Default::default())?;
    let request_ns = elapsed_ns(start);
    lab::atomic_json(&output.join("chunk-receipt.json"), &reply)?;
    let end_tick = server.clock()?;
    validate_advance(start_tick, 0, end_tick)?;
    if reply["chunk_probe"]["owner_thread"] != true
        || lab::observation_tick(&reply["chunk_probe"]["before"])? != start_tick
        || reply["chunk_probe"]["chunk"] != json!([TARGET, TARGET])
    {
        return Err("chunk receipt has the wrong owner, coordinate or console clock".into());
    }
    let window_batches = server.steps(3)?;
    let later = server
        .bridge
        .as_mut()
        .ok_or("chunk witness needs the bridge")?
        .request("chunk_witness", &Default::default())?;
    lab::atomic_json(&output.join("chunk-followup.json"), &later)?;
    let later = &later["chunk_witness"];
    let followup_loads = later["load_events"]
        .as_u64()
        .ok_or("missing delayed load count")?;
    if later["owner_thread"] != true || lab::observation_tick(&later["state"])? != server.clock()? {
        return Err("delayed chunk witness has the wrong owner or console clock".into());
    }
    let flush_ns = server.save()?;
    let process = server.exit(false)?;
    let persisted = region_has_target(&dir)?;
    let probe = &reply["chunk_probe"];
    let loads = probe["load_events"]
        .as_u64()
        .ok_or("missing chunk-load sentinel")?;
    let result = json!({"force_load":force_load,"request_ns":request_ns,"flush_ns":flush_ns,
        "process":process,"probe":probe,"followup":later,"window_batches":window_batches,
        "persisted_target_record":persisted});
    lab::atomic_json(&output.join("trial.json"), &result)?;
    validate(
        &probe["before"],
        &probe["after"],
        loads,
        persisted,
        force_load,
    )?;
    validate_followup(&probe["after"], &later["state"], followup_loads, force_load)?;
    Ok(result)
}

pub(super) fn campaign(environment: &Environment, output: &Path) -> LabResult<()> {
    let start = Instant::now();
    let jobs: Vec<_> = (1..=3)
        .flat_map(|i| {
            [false, true].map(|fault| {
                (
                    format!(
                        "{i}-{}",
                        if fault {
                            "allocating_read"
                        } else {
                            "nonloading_read"
                        }
                    ),
                    fault,
                )
            })
        })
        .collect();
    let results = bounded_map(&jobs, 2, |(name, fault)| {
        eprintln!("Running chunk control {name}");
        trial(environment, &output.join(name), *fault)
    });
    let mut attempts = Vec::new();
    let mut errors = Vec::new();
    for ((name, _), result) in jobs.into_iter().zip(results) {
        if let Err(error) = &result {
            errors.push(format!("{name}: {error}"));
        }
        attempts.push(
            json!({"trial":name,"evidence":result.as_ref().ok(),"error":result.as_ref().err()}),
        );
    }
    lab::atomic_json(
        &output.join("chunk-campaign.json"),
        &json!({
            "trials":attempts,"campaign_wall_ns":elapsed_ns(start),"qualified":false,
        }),
    )?;
    if !errors.is_empty() {
        return Err(errors.join(" | "));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn unloaded() -> Value {
        json!({"tick":"12","resident":false,"holder":false,"forced":false,"debug":"absent"})
    }

    #[test]
    fn innocent_observer_requires_no_chunk_or_ticket_change() {
        let before = unloaded();
        assert!(validate(&before, &before, 0, false, false).is_ok());
        let mut after = before.clone();
        after["holder"] = json!(true);
        assert!(
            validate(&before, &after, 0, false, false)
                .unwrap_err()
                .contains("neutrality")
        );
        assert!(validate(&before, &before, 1, false, false).is_err());
        assert!(validate(&before, &before, 0, true, false).is_err());
    }

    #[test]
    fn load_fault_needs_live_event_and_disk_corroboration_at_same_tick() {
        let before = unloaded();
        let mut after = before.clone();
        after["resident"] = json!(true);
        after["holder"] = json!(true);
        after["debug"] = json!("loaded");
        assert!(validate(&before, &after, 1, true, true).is_ok());
        assert!(validate(&before, &after, 0, true, true).is_err());
        assert!(validate(&before, &after, 1, false, true).is_err());
        after["tick"] = json!("13");
        assert!(
            validate(&before, &after, 1, true, true)
                .unwrap_err()
                .contains("clock")
        );
    }

    #[test]
    fn a_delayed_holder_or_load_event_fails_the_nonloading_window() {
        let immediate = unloaded();
        let mut later = immediate.clone();
        later["tick"] = json!("15");
        assert!(validate_followup(&immediate, &later, 0, false).is_ok());
        assert!(validate_followup(&immediate, &later, 1, false).is_err());
        later["holder"] = json!(true);
        assert!(validate_followup(&immediate, &later, 0, false).is_err());
    }
}

use crate::LabResult;
use serde_json::Value;
#[derive(Clone, Copy, PartialEq, Eq)]
struct Counters {
    tick: u64,
    input: u64,
    output: u64,
    energy: u64,
}
fn number(v: &Value, key: &str) -> LabResult<u64> {
    let s = v
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing receipt {key}"))?;
    let n = s
        .parse::<u64>()
        .map_err(|_| format!("invalid receipt {key}"))?;
    if n.to_string() != s {
        return Err(format!("noncanonical receipt {key}"));
    }
    Ok(n)
}
fn counters(v: &Value) -> LabResult<Counters> {
    let side = v
        .pointer("/machine/sides/0")
        .ok_or("missing machine capability")?;
    if side["side"] != "unsided" || side["energy_unit"] != "FE" {
        return Err("unsupported side/unit binding".into());
    }
    let slots = side["slots"].as_array().ok_or("missing slots")?;
    if slots.len() != 3 {
        return Err("unexpected slot binding".into());
    }
    let (mut input, mut output) = (0, 0);
    for slot in slots {
        let n = slot["count"].as_u64().ok_or("missing stack count")?;
        if n > 64 {
            return Err("unexpected item/count".into());
        }
        match (slot["id"].as_str(), n) {
            (Some("minecraft:air"), 0) => {}
            (Some("minecraft:iron_ore"), 1..=64) => input += n,
            (Some("mekanism:dust_iron"), 1..=64) => output += n,
            _ => return Err("unexpected item/count or energy item".into()),
        }
    }
    Ok(Counters {
        tick: crate::lab::observation_tick(v)?,
        input,
        output,
        energy: number(side, "energy")?,
    })
}
pub fn analyze(observations: &[Value], mode: &str) -> LabResult<Value> {
    let first = observations.first().ok_or("empty production evidence")?;
    let mut previous = counters(&first["state"])?;
    if first["action"] != "setup_machine"
        || previous.input != 0
        || previous.output != 0
        || previous.energy != 0
    {
        return Err("production requires empty fresh setup".into());
    }
    let (
        mut inserted,
        mut consumed,
        mut produced,
        mut added,
        mut extracted,
        mut received,
        mut removed,
        mut spent,
        mut ticks,
    ) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    for observation in &observations[1..] {
        let state = &observation["state"];
        let current = counters(state)?;
        let action = observation["action"].as_str().ok_or("missing action")?;
        if action == "step" {
            let n = current
                .tick
                .checked_sub(previous.tick)
                .ok_or("work tick regression")?;
            let samples: Vec<crate::capture::TickSample> =
                serde_json::from_value(observation["tick_samples"].clone())
                    .map_err(|e| e.to_string())?;
            crate::capture::validate_batch(previous.tick, n, mode, &samples, state)?;
            for sample in samples {
                let next = counters(&sample.state)?;
                let used = previous
                    .input
                    .checked_sub(next.input)
                    .ok_or("recipe input increased during tick")?;
                let made = next
                    .output
                    .checked_sub(previous.output)
                    .ok_or("recipe output decreased during tick")?;
                if made != used * 2 {
                    return Err("recipe production balance mismatch".into());
                }
                consumed += used;
                produced += made;
                spent += previous
                    .energy
                    .checked_sub(next.energy)
                    .ok_or("undeclared energy source during tick")?;
                previous = next;
            }
            ticks += n;
            continue;
        }
        if current.tick != previous.tick {
            return Err("mutation advanced undeclared ticks".into());
        }
        match action {
            "insert_input" => {
                if current.output != previous.output
                    || current.energy != previous.energy
                    || current.input <= previous.input
                {
                    return Err("input mutation balance mismatch".into());
                }
                inserted += current.input - previous.input;
            }
            "charge_machine" | "drain_machine" => {
                if current.input != previous.input || current.output != previous.output {
                    return Err("energy action changed items".into());
                }
                let r = &observation["energy_receipt"];
                if r["unit"] != "FE"
                    || number(r, "before")? != previous.energy
                    || number(r, "after")? != current.energy
                {
                    return Err("energy receipt state mismatch".into());
                }
                let delta = if action == "charge_machine" {
                    current.energy.checked_sub(previous.energy)
                } else {
                    previous.energy.checked_sub(current.energy)
                }
                .ok_or("energy receipt direction mismatch")?;
                if number(
                    r,
                    if action == "charge_machine" {
                        "accepted"
                    } else {
                        "removed"
                    },
                )? != delta
                    || number(r, "requested")? < delta
                {
                    return Err("energy receipt quantity mismatch".into());
                }
                if action == "charge_machine" {
                    received += delta;
                } else {
                    removed += delta;
                }
            }
            "fixture_block_output" => {
                if observation["fixture_level"] != "administrative_nbt"
                    || previous.output != 2
                    || current.output != 64
                    || previous.input != current.input
                    || previous.energy != current.energy
                {
                    return Err("output fixture balance mismatch".into());
                }
                added += 62;
            }
            "extract_output" => {
                let r = &observation["extraction_receipt"];
                let delta = previous
                    .output
                    .checked_sub(current.output)
                    .ok_or("extraction direction mismatch")?;
                if delta == 0
                    || r["id"] != "mekanism:dust_iron"
                    || r["count"].as_u64() != Some(delta)
                    || previous.input != current.input
                    || previous.energy != current.energy
                {
                    return Err("extraction receipt mismatch".into());
                }
                extracted += delta;
            }
            "read" | "save" => {
                if current != previous {
                    return Err(format!("{action} changed work counters"));
                }
            }
            _ => return Err(format!("unsupported accounting action {action}")),
        }
        previous = current;
    }
    if consumed == 0 || ticks == 0 {
        return Err("no actual recipe production".into());
    }
    if inserted != consumed + previous.input
        || produced + added != extracted + previous.output
        || received != removed + spent + previous.energy
    {
        return Err("work ledger does not close".into());
    }
    Ok(
        serde_json::json!({"scope":"fixed enrichment pilot, public unsided bindings, FE","ore_inserted":inserted,"ore_consumed":consumed,"dust_produced":produced,"fixture_dust_added":added,"dust_extracted":extracted,"energy_received_fe":received,"energy_removed_fe":removed,"energy_consumed_fe":spent,"advanced_ticks":ticks,"final_energy_fe":previous.energy,"final_input":previous.input,"final_output":previous.output,"optimization_proven":false}),
    )
}

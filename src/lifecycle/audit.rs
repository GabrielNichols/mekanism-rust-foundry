use crate::LabResult;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub(super) struct Checkpoint {
    pub tick: u64,
    pub progress: u32,
    pub ore: u32,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(super) enum Verdict {
    Accepted,
    Divergence {
        witness: &'static str,
        field: &'static str,
        expected: u32,
        observed: u32,
    },
}

pub(super) fn classify(
    before: Checkpoint,
    observed: Checkpoint,
    after: Checkpoint,
    saved: Checkpoint,
) -> LabResult<Verdict> {
    if before.tick != observed.tick || before.tick != after.tick {
        return Err("audit clock changed across the frozen read".into());
    }
    if saved.progress != after.progress || saved.ore != after.ore {
        return Err("persisted checkpoint does not corroborate the post-read world".into());
    }
    for (witness, checkpoint) in [("observer", observed), ("post_read_world", after)] {
        for (field, expected, observed) in [
            ("progress", before.progress, checkpoint.progress),
            ("ore", before.ore, checkpoint.ore),
        ] {
            if expected != observed {
                return Ok(Verdict::Divergence {
                    witness,
                    field,
                    expected,
                    observed,
                });
            }
        }
    }
    Ok(Verdict::Accepted)
}

use super::{
    CUT, Environment, Observer, Server, bounded_map, copy_tree, elapsed_ns, lab, launch, nbt,
    prepare_seed, pulse, validate_advance, validate_machine_with_ore,
};
use serde_json::{Value, json};
use std::{fs, path::Path, time::Instant};

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Fault {
    Clean,
    Stale,
    Inventory,
}

impl Fault {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::Stale => "stale_snapshot",
            Self::Inventory => "after_read_inventory",
        }
    }

    fn expected(self) -> Verdict {
        match self {
            Self::Clean => Verdict::Accepted,
            Self::Stale => Verdict::Divergence {
                witness: "observer",
                field: "progress",
                expected: CUT,
                observed: 0,
            },
            Self::Inventory => Verdict::Divergence {
                witness: "post_read_world",
                field: "ore",
                expected: 1,
                observed: 2,
            },
        }
    }
}

fn console_checkpoint(server: &mut Server) -> LabResult<(Checkpoint, String)> {
    let tick = server.clock()?;
    let snbt = server.snapshot()?;
    let progress = server.live_int("progress")?;
    let ore = server.live_int("items[0].item.count")?;
    validate_advance(tick, 0, server.clock()?)?;
    Ok((
        Checkpoint {
            tick,
            progress,
            ore,
        },
        snbt,
    ))
}

const COUNT_PATH: &str = "/value/items/value/0/value/item/value/count/value";

fn from_nbt(value: &Value, tick: u64) -> LabResult<Checkpoint> {
    let number = |path| -> LabResult<u32> {
        value
            .pointer(path)
            .and_then(Value::as_str)
            .and_then(|raw| raw.parse().ok())
            .ok_or_else(|| format!("missing or invalid typed audit field {path}"))
    };
    let progress = number("/value/progress/value")?;
    let ore = number(COUNT_PATH)?;
    validate_machine_with_ore(value, progress, false, ore)?;
    Ok(Checkpoint {
        tick,
        progress,
        ore,
    })
}

fn receipt(server: &mut Server, phase: &str) -> LabResult<(Value, u64)> {
    let args = json!({"checkpoint":phase});
    let bridge = server
        .bridge
        .as_mut()
        .ok_or("audit requires an enabled bridge")?;
    let start = Instant::now();
    let value = bridge.request("audit_read", args.as_object().unwrap())?;
    let ns = elapsed_ns(start);
    lab::atomic_json(
        &server.log.with_extension(format!("{phase}.receipt.json")),
        &value,
    )?;
    Ok((value, ns))
}

fn check_receipt(receipt: &Value, stage: &str) -> LabResult<Checkpoint> {
    if receipt["audit"]
        != json!({"checkpoint":stage,"owner_thread":true,"phase":"server_post_command"})
    {
        return Err(format!(
            "unexpected audit phase or owner: {}",
            receipt["audit"]
        ));
    }
    let state = &receipt["state"];
    let raw_tick = state["tick"].as_str().ok_or("missing observer tick")?;
    let tick: u64 = raw_tick.parse().map_err(|_| "invalid observer tick")?;
    if tick.to_string() != raw_tick {
        return Err("noncanonical observer tick".into());
    }
    let machine = &state["machine"];
    crate::witness::inventory(machine)?;
    let point = from_nbt(&machine["nbt"], tick)?;
    if machine["progress"] != point.progress || machine["iron_ore"] != point.ore {
        return Err("observer aliases differ from its typed NBT".into());
    }
    Ok(point)
}

struct Trial {
    report: Value,
    expected_disk: Value,
    actual_disk: Value,
}

fn trial(environment: &Environment, output: &Path, seed: &Path, fault: Fault) -> LabResult<Trial> {
    let instance = output.join("instance");
    fs::create_dir_all(&instance).map_err(|error| error.to_string())?;
    for name in ["world", "config"] {
        copy_tree(&seed.join(name), &instance.join(name)).map_err(|error| error.to_string())?;
    }
    let mut server = launch(
        environment,
        &instance,
        output.join("server"),
        Observer::Audit(fault),
        100,
    )?;
    server.expect("save-off", "Automatic saving is now disabled")?;

    let (initial, _) = console_checkpoint(&mut server)?;
    let (prime, _) = receipt(&mut server, "initial")?;
    if initial.progress != 0 || initial.ore != 1 || check_receipt(&prime, "initial")? != initial {
        return Err("audit initial receipt differs from the independent seed checkpoint".into());
    }
    let batches = pulse(&mut server, CUT)?;

    let timer = Instant::now();
    let (before, before_snbt) = console_checkpoint(&mut server)?;
    let pre_read_ns = elapsed_ns(timer);
    if before.progress != CUT || before.ore != 1 {
        return Err("audit never reached the independent processing checkpoint".into());
    }
    let (read, read_ns) = receipt(&mut server, "cut")?;
    let timer = Instant::now();
    let (after, after_snbt) = console_checkpoint(&mut server)?;
    let post_read_ns = elapsed_ns(timer);
    lab::atomic_json(
        &output.join("console-witness.json"),
        &json!({
            "before":before,"after":after,"before_snbt":before_snbt,"after_snbt":after_snbt,
        }),
    )?;

    let flush_ns = server.save()?;
    let process = server.exit(false)?;
    let raw = nbt::block_entity(&instance.join("world/region/r.0.0.mca"), 0, 80, 0)?;
    lab::atomic_json(&output.join("persisted-nbt.json"), &raw)?;
    let actual_disk = nbt::persisted_projection(&raw)?;
    let saved = from_nbt(&actual_disk, after.tick)?;
    let observed = check_receipt(&read, "cut")?;
    let timer = Instant::now();
    let verdict = classify(before, observed, after, saved)?;
    let classifier_ns = elapsed_ns(timer);
    let report = json!({
        "fault":fault.name(),"verdict":verdict,"expected_verdict":fault.expected(),
        "before":before,"observer":observed,"after":after,
        "persisted":{"progress":saved.progress,"ore":saved.ore},
        "timings_ns":{"console_before":pre_read_ns,"observer_round_trip":read_ns,
            "console_after":post_read_ns,"flush":flush_ns,"classification":classifier_ns},
        "process":process,"batches":batches,
    });
    lab::atomic_json(&output.join("trial.json"), &report)?;
    if verdict != fault.expected() {
        return Err(format!("unexpected observer verdict: {verdict:?}"));
    }
    if (before_snbt != after_snbt) != (fault == Fault::Inventory) {
        return Err("full console snapshot disagrees with the expected world effect".into());
    }
    let prime_nbt = &prime["state"]["machine"]["nbt"];
    let observer_nbt = &read["state"]["machine"]["nbt"];
    if fault == Fault::Stale && observer_nbt != prime_nbt {
        return Err("stale observer did not replay the captured initial machine".into());
    }
    Ok(Trial {
        report,
        actual_disk,
        expected_disk: observer_nbt.clone(),
    })
}

pub(super) fn campaign(environment: &Environment, output: &Path) -> LabResult<()> {
    let start = Instant::now();
    let (seed, _) = prepare_seed(environment, output)?;
    let jobs: Vec<_> = (1..=3)
        .flat_map(|repeat| {
            [Fault::Clean, Fault::Stale, Fault::Inventory]
                .map(|fault| (format!("{repeat}-{}", fault.name()), fault))
        })
        .collect();
    let outcomes = bounded_map(&jobs, 3, |(name, fault)| {
        let start_ns = elapsed_ns(start);
        eprintln!("Running live observer audit {name}");
        let result = trial(environment, &output.join(name), &seed, *fault);
        let timing = json!({"trial":name,"start_ns":start_ns,"end_ns":elapsed_ns(start),
            "error":result.as_ref().err()});
        (result, timing)
    });
    let attempts: Vec<_> = outcomes.iter().map(|(_, timing)| timing).collect();
    lab::atomic_json(&output.join("attempts.json"), &attempts)?;
    let mut baseline = None;
    let mut reports = Vec::new();
    for ((name, fault), (result, _)) in jobs.into_iter().zip(outcomes) {
        let trial = result.map_err(|error| format!("{name}: {error}"))?;
        let baseline = baseline.get_or_insert_with(|| trial.actual_disk.clone());
        let mut expected_disk = baseline.clone();
        if fault == Fault::Inventory {
            *expected_disk
                .pointer_mut(COUNT_PATH)
                .ok_or("missing baseline ore count")? = json!("2");
        }
        if trial.actual_disk != expected_disk
            || (fault != Fault::Stale && &trial.expected_disk != baseline)
        {
            return Err(format!(
                "{name}: full typed disk or observer NBT diverged from the intended effect"
            ));
        }
        reports.push(json!({"trial":name,"evidence":trial.report}));
    }
    lab::atomic_json(
        &output.join("audit-campaign.json"),
        &json!({
            "scope":"live_enrichment_chamber_observer_audit","campaign_wall_ns":elapsed_ns(start),
            "trials":reports,"qualified":false,"port_equivalence_proven":false,"optimization_proven":false,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLEAN: Checkpoint = Checkpoint {
        tick: 203,
        progress: 73,
        ore: 1,
    };

    #[test]
    fn relabelled_stale_snapshot_fails_against_the_pre_read_witness() {
        let stale = Checkpoint {
            progress: 0,
            ..CLEAN
        };
        assert_eq!(
            classify(CLEAN, stale, CLEAN, CLEAN).unwrap(),
            Verdict::Divergence {
                witness: "observer",
                field: "progress",
                expected: 73,
                observed: 0,
            }
        );
    }

    #[test]
    fn valid_reply_followed_by_inventory_mutation_fails_against_the_world() {
        let mutated = Checkpoint { ore: 2, ..CLEAN };
        assert_eq!(
            classify(CLEAN, CLEAN, mutated, mutated).unwrap(),
            Verdict::Divergence {
                witness: "post_read_world",
                field: "ore",
                expected: 1,
                observed: 2,
            }
        );
    }

    #[test]
    fn phase_drift_and_unconfirmed_disk_state_cannot_count_as_detected_faults() {
        let moved = Checkpoint { tick: 204, ..CLEAN };
        let mutated = Checkpoint { ore: 2, ..CLEAN };
        assert!(
            classify(CLEAN, CLEAN, moved, CLEAN)
                .unwrap_err()
                .contains("clock")
        );
        assert!(
            classify(CLEAN, CLEAN, mutated, CLEAN)
                .unwrap_err()
                .contains("persisted")
        );
        assert_eq!(
            classify(CLEAN, CLEAN, CLEAN, CLEAN).unwrap(),
            Verdict::Accepted
        );
    }
}

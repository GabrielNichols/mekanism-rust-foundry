use crate::LabResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TickSample {
    pub tick: String,
    pub phase: String,
    pub owner_thread: bool,
    pub state: Value,
}
pub fn validate_batch(
    start: u64,
    ticks: u64,
    mode: &str,
    samples: &[TickSample],
    end: &Value,
) -> LabResult<()> {
    if ticks == 0 || ticks > 20 {
        return Err("invalid capture tick budget".into());
    }
    let expected_count = match mode {
        "every_tick" => ticks as usize,
        "terminal" => 1,
        _ => return Err("unknown observer mode".into()),
    };
    if samples.len() != expected_count {
        return Err("capture sample count mismatch".into());
    }
    for (index, sample) in samples.iter().enumerate() {
        let expected = start
            .checked_add(if mode == "terminal" {
                ticks
            } else {
                index as u64 + 1
            })
            .ok_or("tick overflow")?;
        if sample.tick != expected.to_string() {
            return Err("tick continuity mismatch".into());
        }
        if sample.phase != "end_server_tick" {
            return Err("capture phase mismatch".into());
        }
        if !sample.owner_thread {
            return Err("capture thread is not owner".into());
        }
        if sample.state.get("tick").and_then(Value::as_str) != Some(sample.tick.as_str()) {
            return Err("state tick differs from envelope".into());
        }
    }
    if samples.last().map(|s| &s.state) != Some(end) {
        return Err("final state differs from last capture".into());
    }
    Ok(())
}

pub(crate) fn validate_reply(start: u64, ticks: u64, mode: &str, reply: &Value) -> LabResult<()> {
    let samples: Vec<TickSample> = serde_json::from_value(reply["tick_samples"].clone())
        .map_err(|error| format!("bridge samples: {error}"))?;
    validate_batch(start, ticks, mode, &samples, &reply["state"])?;
    crate::measurement::validate(&reply["measurement"], start, ticks, mode)
}

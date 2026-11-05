use crate::LabResult;
use serde_json::Value;
fn number(value: &Value, key: &str) -> LabResult<u64> {
    let raw = value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing measurement {key}"))?;
    let n = raw
        .parse::<u64>()
        .map_err(|_| format!("invalid measurement {key}"))?;
    if n.to_string() != raw {
        return Err(format!("noncanonical measurement {key}"));
    }
    Ok(n)
}
pub fn validate(value: &Value, start: u64, ticks: u64, mode: &str) -> LabResult<()> {
    if !(1..=20).contains(&ticks) || !matches!(mode, "every_tick" | "terminal") {
        return Err("invalid measurement budget/mode".into());
    }
    for key in [
        "batch_wall_ns",
        "process_cpu_ns",
        "gc_pause_ms",
        "gc_collections",
        "heap_used_bytes",
    ] {
        number(value, key)?;
    }
    let rows = value["ticks"]
        .as_array()
        .ok_or("missing measurement ticks")?;
    if rows.len() != ticks as usize {
        return Err("measurement count mismatch".into());
    }
    for (index, row) in rows.iter().enumerate() {
        if number(row, "tick")? != start.checked_add(index as u64 + 1).ok_or("tick overflow")? {
            return Err("measurement tick mismatch".into());
        }
        if number(row, "bridge_pre_to_post_ns")? == 0 {
            return Err("zero measured tick duration".into());
        }
        let capture = number(row, "snapshot_ns")?;
        if (mode == "every_tick" || index + 1 == rows.len()) != (capture > 0) {
            return Err("snapshot timing inconsistent with observer mode".into());
        }
    }
    Ok(())
}
pub fn summarize(records: &[Value]) -> LabResult<Value> {
    if records.is_empty() {
        return Err("empty measurements".into());
    }
    let mut tick_ns = Vec::new();
    let mut capture_ns = Vec::new();
    let mut wall = 0u64;
    let mut cpu = 0u64;
    let mut gc = 0u64;
    let mut count = 0u64;
    let mut heap = 0u64;
    for record in records {
        let m = &record["measurement"];
        wall = wall
            .checked_add(number(m, "batch_wall_ns")?)
            .ok_or("wall overflow")?;
        cpu = cpu
            .checked_add(number(m, "process_cpu_ns")?)
            .ok_or("CPU overflow")?;
        gc = gc
            .checked_add(number(m, "gc_pause_ms")?)
            .ok_or("GC overflow")?;
        count = count
            .checked_add(number(m, "gc_collections")?)
            .ok_or("GC count overflow")?;
        heap = heap.max(number(m, "heap_used_bytes")?);
        for row in m["ticks"].as_array().ok_or("missing ticks")? {
            tick_ns.push(number(row, "bridge_pre_to_post_ns")?);
            let n = number(row, "snapshot_ns")?;
            if n > 0 {
                capture_ns.push(n);
            }
        }
    }
    fn percentiles(mut v: Vec<u64>) -> LabResult<Value> {
        if v.is_empty() {
            return Err("empty duration series".into());
        }
        v.sort_unstable();
        let p = |q: usize| v[(v.len() * q).div_ceil(100) - 1];
        Ok(serde_json::json!({"p50":p(50),"p95":p(95),"p99":p(99)}))
    }
    Ok(
        serde_json::json!({"ticks":tick_ns.len(),"snapshots":capture_ns.len(),"bridge_pre_to_post_ns":percentiles(tick_ns)?,"snapshot_ns":percentiles(capture_ns)?,"batch_wall_ns":wall.to_string(),"process_cpu_ns":cpu.to_string(),"gc_pause_ms":gc.to_string(),"gc_collections":count.to_string(),"max_sampled_heap_used_bytes":heap.to_string(),"optimization_proven":false}),
    )
}

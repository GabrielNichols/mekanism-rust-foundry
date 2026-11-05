use mek_lab::measurement::{summarize, validate};
use serde_json::{Value, json};
fn batch() -> Value {
    json!({"ticks":[{"tick":"11","bridge_pre_to_post_ns":"10","snapshot_ns":"3"},{"tick":"12","bridge_pre_to_post_ns":"20","snapshot_ns":"4"}],"batch_wall_ns":"50000000","process_cpu_ns":"2000","gc_pause_ms":"0","gc_collections":"0","heap_used_bytes":"1000"})
}
#[test]
fn validates_actual_tick_measurements() {
    validate(&batch(), 10, 2, "every_tick").unwrap();
}
#[test]
fn missing_measurement_is_not_zero() {
    let mut v = batch();
    v.as_object_mut().unwrap().remove("process_cpu_ns");
    assert!(
        validate(&v, 10, 2, "every_tick")
            .unwrap_err()
            .contains("process_cpu_ns")
    );
}
#[test]
fn negative_counter_is_invalid() {
    let mut v = batch();
    v["gc_pause_ms"] = json!("-1");
    assert!(
        validate(&v, 10, 2, "every_tick")
            .unwrap_err()
            .contains("gc_pause_ms")
    );
}
#[test]
fn timings_must_cover_all_ticks() {
    let mut v = batch();
    v["ticks"].as_array_mut().unwrap().pop();
    assert!(
        validate(&v, 10, 2, "every_tick")
            .unwrap_err()
            .contains("count")
    );
}
#[test]
fn terminal_mode_cannot_claim_intermediate_snapshots() {
    assert!(
        validate(&batch(), 10, 2, "terminal")
            .unwrap_err()
            .contains("snapshot")
    );
}
#[test]
fn summary_keeps_wall_and_simulation_cost_separate() {
    let v = vec![json!({"measurement":batch()})];
    let s = summarize(&v).unwrap();
    assert_eq!(s["ticks"], 2);
    assert_eq!(s["bridge_pre_to_post_ns"]["p50"], 10);
    assert_eq!(s["bridge_pre_to_post_ns"]["p95"], 20);
    assert_eq!(s["snapshot_ns"]["p99"], 4);
    assert_eq!(s["batch_wall_ns"], "50000000");
    assert_eq!(s["optimization_proven"], false);
}

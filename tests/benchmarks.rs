use mek_lab::benchmark::{Study, summarize};
use serde_json::json;
fn study() -> Study {
    let trials=[210u64,190,200,220,180].map(|time|json!({"reference_ns":time,"candidate_ns":time/2,"reference_units":1000,"candidate_units":1000,"reference_output_sha256":"cd".repeat(32),"candidate_output_sha256":"cd".repeat(32)}));
    serde_json::from_value(
        json!({"workload_sha256":"ab".repeat(32),"phase":"steady_state","warmup_units":100,
        "trials":trials}),
    )
    .unwrap()
}
#[test]
fn descriptive_ratio_does_not_claim_optimization_proof() {
    let s = summarize(&study()).unwrap();
    assert_eq!(s.paired_trials, 5);
    assert_eq!(s.reference_lower_median_ns, 200);
    assert_eq!(s.candidate_lower_median_ns, 100);
    assert_eq!(s.descriptive_ratio, 2.0);
    assert!(!s.optimization_proven);
}
#[test]
fn less_work_cannot_count_as_faster() {
    let mut s = study();
    s.trials[0].candidate_units = 999;
    assert!(summarize(&s).unwrap_err().contains("units"));
}
#[test]
fn different_results_block_speed_comparison() {
    let mut s = study();
    s.trials[0].candidate_output_sha256 = "ef".repeat(32);
    assert!(summarize(&s).unwrap_err().contains("output"));
}
#[test]
fn steady_state_requires_warmup() {
    let mut s = study();
    s.warmup_units = 0;
    assert!(summarize(&s).unwrap_err().contains("warmup"));
}
#[test]
fn one_run_is_not_a_repeated_study() {
    let mut s = study();
    s.trials.truncate(1);
    assert!(summarize(&s).unwrap_err().contains("five"));
}
#[test]
fn zero_duration_is_invalid() {
    let mut s = study();
    s.trials[0].candidate_ns = 0;
    assert!(summarize(&s).unwrap_err().contains("duration"));
}

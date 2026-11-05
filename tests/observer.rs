use mek_lab::observer::check_pair;
use serde_json::{Value, json};
fn pair() -> (Value, Value) {
    let nbt = json!({"tag":10,"value":{"progress":{"tag":3,"value":"0"}}});
    let state = json!({"tick":"0","machine":{"nbt":nbt}});
    let a = json!({"case":{"id":"a","observer_mode":"every_tick","actions":["fixture"]},"artifact_fingerprint":"same","runner_sha256":"same","config":{"mode":"same"},"observations":[{"kind":"observation","sequence":0,"action":"setup_machine","state":state}],"persisted_machine":{"tag":10,"value":{"keepPacked":{"tag":1,"value":"0"},"progress":{"tag":3,"value":"0"}}}});
    let mut b = a.clone();
    b["case"]["observer_mode"] = json!("terminal");
    (a, b)
}
fn stepped_pair() -> (Value, Value) {
    let (mut a, mut b) = pair();
    for run in [&mut a, &mut b] {
        let mode = run["case"]["observer_mode"].as_str().unwrap();
        let state = run["observations"][0]["state"].clone();
        let mut samples = vec![];
        let mut timings = vec![];
        for tick in 1..=2 {
            let captured = mode == "every_tick" || tick == 2;
            let mut s = state.clone();
            s["tick"] = json!(tick.to_string());
            if captured {
                samples.push(json!({"tick":tick.to_string(),"phase":"end_server_tick","owner_thread":true,"state":s}));
            }
            timings.push(json!({"tick":tick.to_string(),"bridge_pre_to_post_ns":"100","snapshot_ns":if captured{"50"}else{"0"}}));
        }
        let final_state = samples.last().unwrap()["state"].clone();
        run["observations"].as_array_mut().unwrap().push(json!({"kind":"observation","sequence":1,"action":"step","state":final_state,"tick_samples":samples,"measurement":{"ticks":timings,"batch_wall_ns":"100000000","process_cpu_ns":"10000","gc_pause_ms":"0","gc_collections":"0","heap_used_bytes":"10000"}}));
    }
    (a, b)
}
#[test]
fn matching_endpoints_and_independent_save_pass_limited_check() {
    let (a, b) = stepped_pair();
    let report = check_pair(&a, &b).unwrap();
    assert_eq!(report["observer_endpoint_equivalent"], true);
    assert_eq!(report["qualified"], false);
}
#[test]
fn missing_capture_blocks_comparison() {
    let (mut a, b) = stepped_pair();
    a["observations"][1]["tick_samples"]
        .as_array_mut()
        .unwrap()
        .remove(0);
    assert!(check_pair(&a, &b).unwrap_err().contains("count"));
}
#[test]
fn observation_modes_require_actual_steps() {
    let (a, b) = pair();
    assert!(check_pair(&a, &b).unwrap_err().contains("step"));
}
#[test]
fn changed_fixture_cannot_qualify_observer() {
    let (a, mut b) = pair();
    b["case"]["actions"] = json!(["other"]);
    assert!(check_pair(&a, &b).unwrap_err().contains("case"));
}
#[test]
fn configuration_drift_blocks_comparison() {
    let (a, mut b) = pair();
    b["config"]["mode"] = json!("different");
    assert!(check_pair(&a, &b).unwrap_err().contains("config"));
}
#[test]
fn artifact_drift_blocks_comparison() {
    let (a, mut b) = pair();
    b["artifact_fingerprint"] = json!("different");
    assert!(check_pair(&a, &b).unwrap_err().contains("artifact"));
}
#[test]
fn changed_live_state_is_not_normalized() {
    let (a, mut b) = pair();
    b["observations"][0]["state"]["machine"]["nbt"]["value"]["progress"]["value"] = json!("1");
    assert!(check_pair(&a, &b).unwrap_err().contains("state"));
}
#[test]
fn changed_saved_state_is_not_normalized() {
    let (a, mut b) = pair();
    b["persisted_machine"]["value"]["progress"]["value"] = json!("1");
    assert!(check_pair(&a, &b).unwrap_err().contains("persisted"));
}
#[test]
fn changed_energy_receipt_blocks_comparison() {
    let (mut a, mut b) = stepped_pair();
    a["observations"][0]["energy_receipt"] = json!({"accepted":"100"});
    b["observations"][0]["energy_receipt"] = json!({"accepted":"99"});
    assert!(check_pair(&a, &b).unwrap_err().contains("metadata"));
}

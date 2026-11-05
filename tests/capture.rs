use mek_lab::capture::{TickSample, validate_batch};
use serde_json::json;
fn samples() -> Vec<TickSample> {
    (8..=10)
        .map(|tick| TickSample {
            tick: tick.to_string(),
            phase: "end_server_tick".into(),
            owner_thread: true,
            state: json!({"tick":tick.to_string(),"progress":tick-7}),
        })
        .collect()
}
#[test]
fn captures_every_tick_in_order() {
    let s = samples();
    validate_batch(7, 3, "every_tick", &s, &s[2].state).unwrap();
}
#[test]
fn missing_intermediate_tick_is_not_complete() {
    let mut s = samples();
    s.remove(1);
    assert!(
        validate_batch(7, 3, "every_tick", &s, &s[1].state)
            .unwrap_err()
            .contains("count")
    );
}
#[test]
fn duplicate_tick_is_rejected() {
    let mut s = samples();
    s[1].tick = "8".into();
    assert!(
        validate_batch(7, 3, "every_tick", &s, &s[2].state)
            .unwrap_err()
            .contains("tick")
    );
}
#[test]
fn wrong_thread_is_rejected() {
    let mut s = samples();
    s[1].owner_thread = false;
    assert!(
        validate_batch(7, 3, "every_tick", &s, &s[2].state)
            .unwrap_err()
            .contains("thread")
    );
}
#[test]
fn state_tick_must_agree_with_envelope() {
    let mut s = samples();
    s[1].state["tick"] = json!("10");
    assert!(
        validate_batch(7, 3, "every_tick", &s, &s[2].state)
            .unwrap_err()
            .contains("state tick")
    );
}
#[test]
fn terminal_mode_is_explicitly_endpoint_only() {
    let mut s = samples();
    let last = s.pop().unwrap();
    validate_batch(
        7,
        3,
        "terminal",
        &[last],
        &json!({"tick":"10","progress":3}),
    )
    .unwrap();
}
#[test]
fn final_state_cannot_differ_from_last_capture() {
    let s = samples();
    assert!(
        validate_batch(7, 3, "every_tick", &s, &json!({"tick":"10","progress":99}))
            .unwrap_err()
            .contains("final")
    );
}

use mek_lab::lab::Case;
use serde_json::{Value, json};
fn case(v: Value) -> Case {
    serde_json::from_value(v).unwrap()
}
fn input() -> Value {
    json!({"id":"machine","client":false,"observer_mode":"every_tick","integration_level":"server_fixture","actions":[{"role":"server","action":"insert_input","args":{"side":"unsided","slot":0,"item":"minecraft:iron_ore","count":1},"expect":[{"pointer":"/machine/progress","equals":0}]}]})
}
#[test]
fn declared_machine_input_is_accepted() {
    case(input()).validate().unwrap();
}
#[test]
fn energy_unit_cannot_be_changed_silently() {
    let mut v = input();
    v["actions"][0]["action"] = json!("charge_machine");
    v["actions"][0]["args"] = json!({"side":"unsided","amount":1000,"unit":"J"});
    assert!(case(v).validate().unwrap_err().contains("argument"));
}
#[test]
fn negative_input_is_rejected() {
    let mut v = input();
    v["actions"][0]["args"]["count"] = json!(-1);
    assert!(case(v).validate().unwrap_err().contains("count"));
}
#[test]
fn unknown_observer_mode_is_rejected() {
    let mut v = input();
    v["observer_mode"] = json!("quietly_skip");
    assert!(case(v).validate().unwrap_err().contains("observer"));
}
#[test]
fn advancing_requires_goal_and_budget() {
    let mut v = input();
    v["actions"][0]["action"] = json!("advance_until");
    v["actions"][0]["args"] = json!({"max_ticks":1000,"batch_ticks":20});
    case(v.clone()).validate().unwrap();
    v["actions"][0]["expect"] = json!([]);
    assert!(case(v).validate().unwrap_err().contains("assertion"));
}
#[test]
fn huge_advance_budget_is_rejected() {
    let mut v = input();
    v["actions"][0]["action"] = json!("advance_until");
    v["actions"][0]["args"] = json!({"max_ticks":1_000_000,"batch_ticks":20});
    assert!(case(v).validate().unwrap_err().contains("budget"));
}
#[test]
fn energy_drain_is_explicit_and_bounded() {
    let mut v = input();
    v["actions"][0]["action"] = json!("drain_machine");
    v["actions"][0]["args"] = json!({"side":"unsided","amount":1000000});
    case(v).validate().unwrap();
}
#[test]
fn output_blocking_is_declared_fixture() {
    let mut v = input();
    v["actions"][0]["action"] = json!("fixture_block_output");
    v["actions"][0]["args"] = json!({});
    case(v).validate().unwrap();
}
#[test]
fn extraction_uses_explicit_slot() {
    let mut v = input();
    v["actions"][0]["action"] = json!("extract_output");
    v["actions"][0]["args"] = json!({"side":"unsided","slot":1,"count":64});
    case(v).validate().unwrap();
}
#[test]
fn actual_client_can_select_recipe_output() {
    let v = json!({"id":"client-output","client":true,"integration_level":"real_client_screen","actions":[{"role":"client","action":"click_slot","args":{"selector":"output"},"expect":[{"pointer":"/cursor/count","equals":2}]}]});
    case(v).validate().unwrap();
}

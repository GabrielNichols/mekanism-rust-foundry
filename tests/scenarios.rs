use mek_lab::lab::Case;
use serde_json::{Value, json};
fn case(value: Value) -> Case {
    serde_json::from_value(value).unwrap()
}
fn valid() -> Value {
    json!({"id":"small-case","client":false,"integration_level":"server_component","actions":[{"role":"server","action":"read","expect":[{"pointer":"/barrel_count","equals":0}]}]})
}
#[test]
fn valid_case_is_accepted() {
    case(valid()).validate().unwrap();
}

#[test]
fn factory_setup_requires_a_boolean_flag_on_a_dedicated_server() {
    let mut v = valid();
    v["actions"][0]["action"] = json!("setup_factory");
    assert!(case(v.clone()).validate().is_err());
    v["actions"][0]["args"] = json!({"blocked": "false"});
    assert!(case(v.clone()).validate().is_err());
    v["actions"][0]["args"] = json!({"blocked": false});
    case(v.clone()).validate().unwrap();
    v["client"] = json!(true);
    assert!(case(v).validate().is_err());
}
#[test]
fn case_with_no_assertions_is_rejected() {
    let mut v = valid();
    v["actions"][0]["expect"] = json!([]);
    assert!(case(v).validate().unwrap_err().contains("assertion"));
}
#[test]
fn client_actions_require_real_client_topology() {
    let mut v = valid();
    v["actions"][0]["role"] = json!("client");
    assert!(case(v).validate().unwrap_err().contains("client"));
}
#[test]
fn mutation_cannot_be_retried_until_it_looks_successful() {
    let mut v = valid();
    v["actions"][0]["action"] = json!("insert_barrel");
    v["actions"][0]["await_condition"] = json!(true);
    assert!(case(v).validate().unwrap_err().contains("retry"));
}
#[test]
fn unknown_action_is_rejected_before_spawning() {
    let mut v = valid();
    v["actions"][0]["action"] = json!("do_anything");
    assert!(case(v).validate().unwrap_err().contains("unsupported"));
}
#[test]
fn reserved_protocol_fields_cannot_be_overridden() {
    let mut v = valid();
    v["actions"][0]["args"] = json!({"sequence":123});
    assert!(case(v).validate().unwrap_err().contains("reserved"));
}
#[test]
fn zero_tick_step_is_rejected() {
    let mut v = valid();
    v["actions"][0]["action"] = json!("step");
    v["actions"][0]["args"] = json!({"ticks":0});
    assert!(case(v).validate().unwrap_err().contains("tick"));
}
#[test]
fn unknown_action_arguments_are_not_silently_ignored() {
    let mut v = valid();
    v["actions"][0]["args"] = json!({"ignored":true});
    assert!(case(v).validate().unwrap_err().contains("argument"));
}
#[test]
fn slot_click_requires_a_valid_selector_before_boot() {
    let mut v = valid();
    v["client"] = json!(true);
    v["actions"][0]["role"] = json!("client");
    v["actions"][0]["action"] = json!("click_slot");
    assert!(case(v).validate().unwrap_err().contains("selector"));
}
#[test]
fn missing_tick_is_not_defaulted_to_zero() {
    assert!(
        mek_lab::lab::observation_tick(&json!({"other":13}))
            .unwrap_err()
            .contains("tick")
    );
}
#[test]
fn malformed_tick_is_not_defaulted_to_zero() {
    assert!(
        mek_lab::lab::observation_tick(&json!({"tick":"invalid"}))
            .unwrap_err()
            .contains("tick")
    );
}
#[test]
fn actual_zero_tick_is_preserved() {
    assert_eq!(
        mek_lab::lab::observation_tick(&json!({"tick":"0"})).unwrap(),
        0
    );
}

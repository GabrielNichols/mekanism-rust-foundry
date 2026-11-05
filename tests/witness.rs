use mek_lab::witness::inventory;
use serde_json::{Value, json};
fn machine() -> Value {
    json!({"nbt":{"tag":10,"value":{"items":{"tag":9,"element_tag":10,"value":[{"tag":10,"value":{"slot":{"tag":1,"value":"0"},"item":{"tag":10,"value":{"id":{"tag":8,"value":"minecraft:iron_ore"},"count":{"tag":3,"value":"3"}}}}}]}}},"sides":[{"side":"unsided","items_available":true,"slots":[{"slot":0,"id":"minecraft:iron_ore","count":3},{"slot":1,"id":"minecraft:air","count":0},{"slot":2,"id":"minecraft:air","count":0}]}],"iron_ore":3,"dust_iron":0})
}
#[test]
fn physical_inventory_agrees_across_representations() {
    inventory(&machine()).unwrap();
}
#[test]
fn stale_capability_cannot_be_corroborated_by_true_nbt() {
    let mut m = machine();
    m["nbt"]["value"]["items"]["value"][0]["value"]["item"]["value"]["count"]["value"] = json!("2");
    assert!(inventory(&m).unwrap_err().contains("capability"));
}
#[test]
fn stale_alias_cannot_pass_when_raw_state_is_true() {
    let mut m = machine();
    m["iron_ore"] = json!(0);
    assert!(inventory(&m).unwrap_err().contains("alias"));
}
#[test]
fn missing_saved_items_do_not_hide_nonempty_capability() {
    let mut m = machine();
    m["nbt"]["value"].as_object_mut().unwrap().remove("items");
    assert!(inventory(&m).unwrap_err().contains("capability"));
}
#[test]
fn absent_items_encode_an_empty_inventory() {
    let mut m = machine();
    m["nbt"]["value"].as_object_mut().unwrap().remove("items");
    m["sides"][0]["slots"][0]["id"] = json!("minecraft:air");
    m["sides"][0]["slots"][0]["count"] = json!(0);
    m["iron_ore"] = json!(0);
    inventory(&m).unwrap();
}
#[test]
fn count_width_is_part_of_the_witness() {
    let mut m = machine();
    m["nbt"]["value"]["items"]["value"][0]["value"]["item"]["value"]["count"]["tag"] = json!(2);
    assert!(inventory(&m).unwrap_err().contains("count"));
}
#[test]
fn duplicate_saved_slot_is_not_a_valid_witness() {
    let mut m = machine();
    let mut entry = m["nbt"]["value"]["items"]["value"][0].clone();
    entry["value"]["item"]["value"]["count"]["value"] = json!("1");
    m["nbt"]["value"]["items"]["value"][0]["value"]["item"]["value"]["count"]["value"] = json!("2");
    m["nbt"]["value"]["items"]["value"]
        .as_array_mut()
        .unwrap()
        .push(entry);
    assert!(inventory(&m).unwrap_err().contains("slot"));
}
#[test]
fn duplicate_capability_slot_is_rejected() {
    let mut m = machine();
    m["sides"][0]["slots"][1]["slot"] = json!(0);
    assert!(inventory(&m).unwrap_err().contains("slot"));
}

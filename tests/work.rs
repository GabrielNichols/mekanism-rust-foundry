use mek_lab::work::analyze;
use serde_json::{Value, json};
fn state(t: u64, input: u64, output: u64, energy: u64) -> Value {
    json!({"tick":t.to_string(),"machine":{"sides":[{"side":"unsided","energy_unit":"FE","energy":energy.to_string(),"slots":[{"id":if output==0{"minecraft:air"}else{"mekanism:dust_iron"},"count":output},{"id":if input==0{"minecraft:air"}else{"minecraft:iron_ore"},"count":input},{"id":"minecraft:air","count":0}]}]}})
}
fn run() -> Vec<Value> {
    let end = state(2, 0, 2, 80);
    vec![
        json!({"action":"setup_machine","state":state(0,0,0,0)}),
        json!({"action":"insert_input","state":state(0,1,0,0)}),
        json!({"action":"charge_machine","state":state(0,1,0,100),"energy_receipt":{"unit":"FE","requested":"100","before":"0","accepted":"100","after":"100"}}),
        json!({"action":"step","state":end,"tick_samples":[{"tick":"1","phase":"end_server_tick","owner_thread":true,"state":state(1,1,0,90)},{"tick":"2","phase":"end_server_tick","owner_thread":true,"state":end}]}),
    ]
}
#[test]
fn accounts_for_real_production_and_energy() {
    let v = analyze(&run(), "every_tick").unwrap();
    assert_eq!(v["ore_consumed"], 1);
    assert_eq!(v["dust_produced"], 2);
    assert_eq!(v["energy_consumed_fe"], 20);
    assert_eq!(v["advanced_ticks"], 2);
    assert_eq!(v["optimization_proven"], false);
}
#[test]
fn fixture_items_do_not_count_as_production() {
    let mut r = run();
    r.push(json!({"action":"fixture_block_output","fixture_level":"administrative_nbt","state":state(2,0,64,80)}));
    r.push(json!({"action":"extract_output","extraction_receipt":{"id":"mekanism:dust_iron","count":64},"state":state(2,0,0,80)}));
    let v = analyze(&r, "every_tick").unwrap();
    assert_eq!(v["dust_produced"], 2);
    assert_eq!(v["fixture_dust_added"], 62);
    assert_eq!(v["dust_extracted"], 64);
}
#[test]
fn wrong_recipe_output_fails() {
    let mut r = run();
    r[3]["state"] = state(2, 0, 3, 80);
    r[3]["tick_samples"][1]["state"] = state(2, 0, 3, 80);
    assert!(analyze(&r, "every_tick").unwrap_err().contains("recipe"));
}
#[test]
fn lying_energy_receipt_fails() {
    let mut r = run();
    r[2]["energy_receipt"]["accepted"] = json!("99");
    assert!(analyze(&r, "every_tick").unwrap_err().contains("receipt"));
}
#[test]
fn undeclared_item_change_fails() {
    let mut r = run();
    r.push(json!({"action":"read","state":state(2,0,3,80)}));
    assert!(analyze(&r, "every_tick").unwrap_err().contains("read"));
}
#[test]
fn energy_unit_is_not_implicitly_converted() {
    let mut r = run();
    r[0]["state"]["machine"]["sides"][0]["energy_unit"] = json!("J");
    assert!(analyze(&r, "every_tick").unwrap_err().contains("unit"));
}
#[test]
fn missing_tick_cannot_prove_work() {
    let mut r = run();
    r[3]["tick_samples"].as_array_mut().unwrap().remove(0);
    assert!(analyze(&r, "every_tick").unwrap_err().contains("count"));
}
#[test]
fn empty_production_cannot_yield_throughput() {
    let mut r = run();
    r.truncate(1);
    assert!(
        analyze(&r, "every_tick")
            .unwrap_err()
            .contains("production")
    );
}
#[test]
fn slot_order_does_not_define_item_identity() {
    let mut r = run();
    for o in &mut r {
        let slots = o["state"]["machine"]["sides"][0]["slots"]
            .as_array_mut()
            .unwrap();
        slots.swap(0, 1);
        if let Some(samples) = o["tick_samples"].as_array_mut() {
            for s in samples {
                s["state"]["machine"]["sides"][0]["slots"]
                    .as_array_mut()
                    .unwrap()
                    .swap(0, 1);
            }
        }
    }
    assert_eq!(analyze(&r, "every_tick").unwrap()["dust_produced"], 2);
}

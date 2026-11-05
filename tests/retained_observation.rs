use serde_json::{Value, json};
#[test]
fn rejected_inventory_witness_keeps_the_original_payload() {
    let dir = tempfile::tempdir().unwrap();
    let observation = json!({"kind":"observation","sequence":0,"action":"read","state":{"tick":"0","machine":{"nbt":{"tag":10,"value":{}},"sides":[{"side":"unsided","items_available":true,"slots":[{"slot":0,"id":"minecraft:air","count":0},{"slot":1,"id":"minecraft:air","count":0},{"slot":2,"id":"minecraft:air","count":0}]}],"iron_ore":1,"dust_iron":0}}});
    let result = mek_lab::lab::retain_observation(dir.path(), &mut vec![], &observation);
    let error = result.expect_err("stale alias must be rejected");
    assert!(error.contains("alias"), "unexpected rejection: {error}");
    let saved: Vec<Value> =
        serde_json::from_slice(&std::fs::read(dir.path().join("observations.json")).unwrap())
            .unwrap();
    assert_eq!(saved, vec![observation]);
}

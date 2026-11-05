use serde_json::json;
#[test]
fn removes_only_unpacked_storage_marker() {
    let saved = json!({"tag":10,"value":{"keepPacked":{"tag":1,"value":"0"},"progress":{"tag":3,"value":"17"}}});
    assert_eq!(
        mek_lab::nbt::persisted_projection(&saved).unwrap(),
        json!({"tag":10,"value":{"progress":{"tag":3,"value":"17"}}})
    );
}
#[test]
fn packed_entity_is_not_normalized() {
    let saved = json!({"tag":10,"value":{"keepPacked":{"tag":1,"value":"1"}}});
    assert!(
        mek_lab::nbt::persisted_projection(&saved)
            .unwrap_err()
            .contains("packed")
    );
}
#[test]
fn wrong_marker_width_is_not_normalized() {
    let saved = json!({"tag":10,"value":{"keepPacked":{"tag":3,"value":"0"}}});
    assert!(
        mek_lab::nbt::persisted_projection(&saved)
            .unwrap_err()
            .contains("marker")
    );
}

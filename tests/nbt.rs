use mek_lab::nbt;
use serde_json::json;
fn fixture() -> Vec<u8> {
    // Compound root, long energy=2^53+1, int x=0, string id="mek", end.
    vec![
        10, 0, 0, 4, 0, 6, b'e', b'n', b'e', b'r', b'g', b'y', 0, 32, 0, 0, 0, 0, 0, 1, 3, 0, 1,
        b'x', 0, 0, 0, 0, 8, 0, 2, b'i', b'd', 0, 3, b'm', b'e', b'k', 0,
    ]
}
#[test]
fn long_precision_and_tag_width_are_preserved() {
    let v = nbt::decode(&fixture()).unwrap();
    assert_eq!(
        v["value"]["energy"],
        json!({"tag":4,"value":"9007199254740993"})
    );
    assert_eq!(v["value"]["x"], json!({"tag":3,"value":"0"}));
    assert_eq!(v["value"]["id"], json!({"tag":8,"value":"mek"}));
}
#[test]
fn truncated_payload_fails_closed() {
    let mut b = fixture();
    b.truncate(12);
    assert!(nbt::decode(&b).unwrap_err().contains("truncated"));
}
#[test]
fn trailing_bytes_are_not_ignored() {
    let mut b = fixture();
    b.push(0);
    assert!(nbt::decode(&b).unwrap_err().contains("trailing"));
}
#[test]
fn negative_array_length_is_invalid() {
    assert!(
        nbt::decode(&[7, 0, 0, 255, 255, 255, 255])
            .unwrap_err()
            .contains("length")
    );
}
#[test]
fn negative_zero_is_bit_exact() {
    assert_eq!(
        nbt::decode(&[6, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0]).unwrap(),
        json!({"tag":6,"value":"8000000000000000"})
    );
}
#[test]
fn duplicate_compound_key_is_invalid() {
    let b = [10, 0, 0, 1, 0, 1, b'a', 0, 1, 0, 1, b'a', 1, 0];
    assert!(nbt::decode(&b).unwrap_err().contains("duplicate"));
}

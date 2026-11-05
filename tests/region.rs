use std::io::Write;
fn region(wrong_x: bool, duplicates: bool) -> Vec<u8> {
    fn name(b: &mut Vec<u8>, tag: u8, key: &str) {
        b.push(tag);
        b.extend_from_slice(&(key.len() as u16).to_be_bytes());
        b.extend_from_slice(key.as_bytes());
    }
    let mut nbt = vec![10, 0, 0];
    name(&mut nbt, 9, "block_entities");
    nbt.push(10);
    nbt.extend_from_slice(&(if duplicates { 2i32 } else { 1i32 }).to_be_bytes());
    for _ in 0..if duplicates { 2 } else { 1 } {
        for (key, n) in [("x", 0i32), ("y", 80), ("z", 0)] {
            if wrong_x && key == "x" {
                name(&mut nbt, 8, key);
                nbt.extend_from_slice(&[0, 1, b'0']);
            } else {
                name(&mut nbt, 3, key);
                nbt.extend_from_slice(&n.to_be_bytes());
            }
        }
        name(&mut nbt, 4, "energy");
        nbt.extend_from_slice(&9007199254740993i64.to_be_bytes());
        nbt.push(0);
    }
    nbt.push(0);
    let mut compressed =
        flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    compressed.write_all(&nbt).unwrap();
    let data = compressed.finish().unwrap();
    let mut file = vec![0; 12288];
    file[..4].copy_from_slice(&[0, 0, 2, 1]);
    file[8192..8196].copy_from_slice(&((data.len() + 1) as u32).to_be_bytes());
    file[8196] = 2;
    file[8197..8197 + data.len()].copy_from_slice(&data);
    file
}
fn read(b: Vec<u8>) -> Result<serde_json::Value, String> {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("r.0.0.mca");
    std::fs::write(&p, b).unwrap();
    mek_lab::nbt::block_entity(&p, 0, 80, 0)
}
#[test]
fn independently_decodes_compressed_region_record() {
    let v = read(region(false, false)).unwrap();
    assert_eq!(v["value"]["energy"]["value"], "9007199254740993");
}
#[test]
fn string_coordinate_cannot_select_target() {
    assert!(read(region(true, false)).is_err());
}
#[test]
fn duplicate_target_is_ambiguous() {
    assert!(read(region(false, true)).unwrap_err().contains("duplicate"));
}
#[test]
fn external_compression_is_not_silently_read() {
    let mut b = region(false, false);
    b[8196] = 130;
    assert!(read(b).unwrap_err().contains("compression"));
}

#[test]
fn crash_before_close_can_leave_a_complete_record_without_sector_padding() {
    let mut bytes = region(false, false);
    let record_len = u32::from_be_bytes(bytes[8192..8196].try_into().unwrap()) as usize;
    bytes.truncate(8196 + record_len);
    let witness = read(bytes).unwrap();
    assert_eq!(witness["value"]["energy"]["value"], "9007199254740993");
}

#[test]
fn crash_that_truncates_the_declared_record_is_rejected() {
    let mut bytes = region(false, false);
    let record_len = u32::from_be_bytes(bytes[8192..8196].try_into().unwrap()) as usize;
    bytes.truncate(8195 + record_len);
    assert!(read(bytes).unwrap_err().contains("truncated region chunk"));
}

#[test]
fn a_record_cannot_read_past_its_allocated_sectors() {
    let mut bytes = region(false, false);
    bytes[8192..8196].copy_from_slice(&4093u32.to_be_bytes());
    bytes.push(0);
    assert!(read(bytes).unwrap_err().contains("record length"));
}

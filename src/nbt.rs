use crate::LabResult;
use serde_json::Value;
use serde_json::{Map, json};
use std::io::Read;
use std::path::Path;
const MAX_BYTES: usize = 32 * 1024 * 1024;
pub fn persisted_projection(saved: &Value) -> LabResult<Value> {
    if saved.get("tag") != Some(&json!(10)) {
        return Err("persisted root not compound".into());
    }
    let mut projected = saved.clone();
    let values = projected
        .get_mut("value")
        .and_then(Value::as_object_mut)
        .ok_or("persisted value not compound")?;
    let marker = values
        .remove("keepPacked")
        .ok_or("missing storage marker")?;
    if marker.get("value") == Some(&json!("1")) {
        return Err("packed entity unsupported".into());
    }
    if marker != json!({"tag":1,"value":"0"}) {
        return Err("unsupported storage marker".into());
    }
    Ok(projected)
}
struct Parser<'a> {
    bytes: &'a [u8],
    at: usize,
    nodes: usize,
}
impl Parser<'_> {
    fn bytes(&mut self, n: usize) -> LabResult<&[u8]> {
        let end = self.at.checked_add(n).ok_or("NBT length overflow")?;
        let value = self
            .bytes
            .get(self.at..end)
            .ok_or("truncated NBT payload")?;
        self.at = end;
        Ok(value)
    }
    fn u8(&mut self) -> LabResult<u8> {
        Ok(self.bytes(1)?[0])
    }
    fn u16(&mut self) -> LabResult<u16> {
        Ok(u16::from_be_bytes(self.bytes(2)?.try_into().unwrap()))
    }
    fn i32(&mut self) -> LabResult<i32> {
        Ok(i32::from_be_bytes(self.bytes(4)?.try_into().unwrap()))
    }
    fn length(&mut self) -> LabResult<usize> {
        let n = self.i32()?;
        if !(0..=1_000_000).contains(&n) {
            return Err("invalid NBT collection length".into());
        }
        Ok(n as usize)
    }
    fn string(&mut self) -> LabResult<String> {
        let len = self.u16()? as usize;
        let b = self.bytes(len)?;
        let mut units = Vec::new();
        let mut p = 0;
        while p < b.len() {
            let c = b[p];
            p += 1;
            let unit = if c & 0x80 == 0 {
                c as u16
            } else if c & 0xe0 == 0xc0 {
                let d = *b.get(p).ok_or("truncated NBT string")?;
                p += 1;
                if d & 0xc0 != 0x80 {
                    return Err("invalid modified UTF-8".into());
                }
                (((c & 31) as u16) << 6) | ((d & 63) as u16)
            } else if c & 0xf0 == 0xe0 {
                let d = *b.get(p).ok_or("truncated NBT string")?;
                let e = *b.get(p + 1).ok_or("truncated NBT string")?;
                p += 2;
                if d & 0xc0 != 0x80 || e & 0xc0 != 0x80 {
                    return Err("invalid modified UTF-8".into());
                }
                (((c & 15) as u16) << 12) | (((d & 63) as u16) << 6) | ((e & 63) as u16)
            } else {
                return Err("unsupported NBT string encoding".into());
            };
            units.push(unit);
        }
        String::from_utf16(&units).map_err(|_| "unsupported unpaired UTF-16 surrogate".into())
    }
    fn payload(&mut self, tag: u8, depth: usize) -> LabResult<Value> {
        self.nodes += 1;
        if depth > 64 || self.nodes > 1_000_000 {
            return Err("NBT resource limit".into());
        }
        let value = match tag {
            1 => json!((self.u8()? as i8).to_string()),
            2 => json!(i16::from_be_bytes(self.bytes(2)?.try_into().unwrap()).to_string()),
            3 => json!(self.i32()?.to_string()),
            4 => json!(i64::from_be_bytes(self.bytes(8)?.try_into().unwrap()).to_string()),
            5 => json!(format!(
                "{:08x}",
                u32::from_be_bytes(self.bytes(4)?.try_into().unwrap())
            )),
            6 => json!(format!(
                "{:016x}",
                u64::from_be_bytes(self.bytes(8)?.try_into().unwrap())
            )),
            8 => json!(self.string()?),
            7 | 11 | 12 => {
                let n = self.length()?;
                let mut values = Vec::with_capacity(n);
                for _ in 0..n {
                    values.push(match tag {
                        7 => (self.u8()? as i8).to_string(),
                        11 => self.i32()?.to_string(),
                        _ => i64::from_be_bytes(self.bytes(8)?.try_into().unwrap()).to_string(),
                    });
                }
                json!(values)
            }
            9 => {
                let child = self.u8()?;
                let n = self.length()?;
                if child > 12 || (child == 0 && n > 0) {
                    return Err("invalid NBT list element tag".into());
                }
                let mut values = Vec::with_capacity(n);
                for _ in 0..n {
                    values.push(self.payload(child, depth + 1)?);
                }
                return Ok(json!({"tag":9,"element_tag":child,"value":values}));
            }
            10 => {
                let mut values = Map::new();
                loop {
                    let child = self.u8()?;
                    if child == 0 {
                        break;
                    }
                    let name = self.string()?;
                    if values.contains_key(&name) {
                        return Err("duplicate NBT compound key".into());
                    }
                    values.insert(name, self.payload(child, depth + 1)?);
                }
                Value::Object(values)
            }
            _ => return Err("unsupported NBT tag".into()),
        };
        Ok(json!({"tag":tag,"value":value}))
    }
}
pub fn decode(bytes: &[u8]) -> LabResult<Value> {
    if bytes.len() > MAX_BYTES {
        return Err("NBT resource limit".into());
    }
    let mut parser = Parser {
        bytes,
        at: 0,
        nodes: 0,
    };
    let tag = parser.u8()?;
    let _name = parser.string()?;
    let value = parser.payload(tag, 0)?;
    if parser.at != bytes.len() {
        return Err("trailing NBT bytes".into());
    }
    Ok(value)
}
pub fn block_entity(region: &Path, x: i32, y: i32, z: i32) -> LabResult<Value> {
    let bytes = std::fs::read(region).map_err(|e| format!("region read: {e}"))?;
    if bytes.len() < 8192 {
        return Err("truncated region header".into());
    }
    let index =
        ((x.div_euclid(16).rem_euclid(32) + z.div_euclid(16).rem_euclid(32) * 32) * 4) as usize;
    let page = ((bytes[index] as usize) << 16)
        | ((bytes[index + 1] as usize) << 8)
        | (bytes[index + 2] as usize);
    let count = bytes[index + 3] as usize;
    if page < 2 || count == 0 {
        return Err("missing region chunk".into());
    }
    let offset = page.checked_mul(4096).ok_or("region offset overflow")?;
    let header = bytes
        .get(offset..offset + 4)
        .ok_or("truncated region chunk")?;
    let size = u32::from_be_bytes(header.try_into().unwrap()) as usize;
    if size < 2 || size > count * 4096 - 4 {
        return Err("invalid region record length".into());
    }
    // A JVM killed before close can leave the last record without sector padding.
    let record = bytes
        .get(offset..offset + 4 + size)
        .ok_or("truncated region chunk")?;
    if record[4] != 2 {
        return Err("unsupported region compression".into());
    }
    let mut decompressed = Vec::new();
    flate2::read::ZlibDecoder::new(&record[5..4 + size])
        .take((MAX_BYTES + 1) as u64)
        .read_to_end(&mut decompressed)
        .map_err(|e| format!("region decompression: {e}"))?;
    let root = decode(&decompressed)?;
    let entities = root
        .pointer("/value/block_entities/value")
        .and_then(Value::as_array)
        .ok_or("missing block_entities list")?;
    let selected: Vec<_> = entities
        .iter()
        .filter(|v| {
            v.pointer("/value/x") == Some(&json!({"tag":3,"value":x.to_string()}))
                && v.pointer("/value/y") == Some(&json!({"tag":3,"value":y.to_string()}))
                && v.pointer("/value/z") == Some(&json!({"tag":3,"value":z.to_string()}))
        })
        .collect();
    if selected.len() != 1 {
        return Err("missing or duplicate target block entity".into());
    }
    Ok(selected[0].clone())
}

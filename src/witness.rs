use crate::LabResult;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
fn payload<'a>(value: &'a Value, tag: u64, label: &str) -> LabResult<&'a Value> {
    if value["tag"].as_u64() != Some(tag) {
        return Err(format!("invalid {label} tag"));
    }
    value
        .get("value")
        .ok_or_else(|| format!("missing {label} payload"))
}
fn add(map: &mut BTreeMap<String, u64>, id: &str, count: u64) -> LabResult<()> {
    if id == "minecraft:air" && count == 0 {
        return Ok(());
    }
    if !id.contains(':') || id == "minecraft:air" || count == 0 || count > i32::MAX as u64 {
        return Err("invalid inventory item/count".into());
    }
    let sum = map.entry(id.into()).or_default();
    *sum = sum.checked_add(count).ok_or("inventory count overflow")?;
    Ok(())
}
pub fn inventory(machine: &Value) -> LabResult<()> {
    let root = payload(&machine["nbt"], 10, "inventory compound")?;
    if !root.is_object() {
        return Err("invalid inventory compound".into());
    }
    let mut stored = BTreeMap::new();
    let mut slots = BTreeSet::new();
    if let Some(list) = root.get("items") {
        let entries = payload(list, 9, "inventory list")?
            .as_array()
            .ok_or("invalid inventory list payload")?;
        if list["element_tag"] != 10 && !(entries.is_empty() && list["element_tag"] == 0) {
            return Err("invalid inventory element tag".into());
        }
        for entry in entries {
            let entry = payload(entry, 10, "inventory entry")?;
            let raw = payload(&entry["slot"], 1, "inventory slot")?
                .as_str()
                .ok_or("invalid inventory slot")?;
            let slot = raw.parse::<i8>().map_err(|_| "invalid inventory slot")?;
            if !(0..=2).contains(&slot) || slot.to_string() != raw || !slots.insert(slot) {
                return Err("duplicate or invalid inventory slot".into());
            }
            let item = payload(&entry["item"], 10, "inventory item")?;
            let id = payload(&item["id"], 8, "item id")?
                .as_str()
                .ok_or("invalid item id")?;
            let raw = payload(&item["count"], 3, "item count")?
                .as_str()
                .ok_or("invalid item count")?;
            let count = raw.parse::<i32>().map_err(|_| "invalid item count")?;
            if count <= 0 || count.to_string() != raw {
                return Err("invalid item count".into());
            }
            add(&mut stored, id, count as u64)?;
        }
    }
    let side = &machine["sides"][0];
    if side["side"] != "unsided" || side["items_available"] != true {
        return Err("inventory capability unavailable".into());
    }
    let entries = side["slots"].as_array().ok_or("missing capability slots")?;
    if entries.len() != 3 {
        return Err("invalid capability slot count".into());
    }
    let mut exposed = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for entry in entries {
        let slot = entry["slot"].as_u64().ok_or("invalid capability slot")?;
        if slot > 2 || !seen.insert(slot) {
            return Err("duplicate or invalid capability slot".into());
        }
        let id = entry["id"].as_str().ok_or("invalid capability item")?;
        let count = entry["count"].as_u64().ok_or("invalid capability count")?;
        add(&mut exposed, id, count)?;
    }
    if exposed != stored {
        return Err("inventory capability differs from NBT items".into());
    }
    for (alias, id) in [
        ("iron_ore", "minecraft:iron_ore"),
        ("dust_iron", "mekanism:dust_iron"),
    ] {
        if machine[alias].as_u64() != Some(stored.get(id).copied().unwrap_or(0)) {
            return Err(format!("inventory alias {alias} differs from NBT"));
        }
    }
    Ok(())
}

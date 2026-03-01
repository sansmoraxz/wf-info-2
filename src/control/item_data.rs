use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;

use serde::Serialize;
use serde_json::Value;

use super::item_data_fetch;
use crate::itemdata;
use crate::itemdata::ProductCategory;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ItemInfo {
    pub name: Option<String>,
    pub unique_name: String,
    pub product_category: Option<String>,
    pub description: Option<String>,
    pub details: Value,
}

// Maps uniqueName/item_type -> all matching ItemInfo variants (multiple productCategory variants may exist)
static ITEM_INDEX: OnceLock<HashMap<String, Vec<ItemInfo>>> = OnceLock::new();

pub(crate) fn lookup_item_info(item_type: &str, category: Option<&str>) -> Option<ItemInfo> {
    let index = ITEM_INDEX.get_or_init(build_item_index);
    let entries = index.get(item_type)?;
    if let Some(cat) = category.and_then(category_to_product_category) {
        if let Some(found) = entries
            .iter()
            .find(|info| info.product_category.as_deref() == Some(cat))
        {
            return Some(found.clone());
        }
    }
    // fallback: first entry
    entries.first().cloned()
}

fn category_to_product_category(cat: &str) -> Option<&'static str> {
    match cat {
        "suits" => Some("Suits"),
        "long_guns" => Some("LongGuns"),
        "pistols" => Some("Pistols"),
        "melee" => Some("Melee"),
        "space_suits" => Some("SpaceSuits"),
        "space_guns" => Some("SpaceGuns"),
        "space_melee" => Some("SpaceMelee"),
        "raw_upgrades" => Some("RawUpgrades"),
        "upgrades" => Some("Upgrades"),
        "recipes" | "pending_recipes" => Some("Recipes"),
        _ => None,
    }
}

fn build_item_index() -> HashMap<String, Vec<ItemInfo>> {
    let mut index: HashMap<String, Vec<ItemInfo>> = HashMap::new();

    let read_cached = |file: &str| -> Option<String> {
        item_data_fetch::cached_path(file)
            .ok()
            .and_then(|p| fs::read_to_string(p).ok())
    };

    let mut push_info = |v: &Value, product_category: Option<String>| {
        let Some(unique_name) = v.get("uniqueName").and_then(Value::as_str) else {
            return;
        };
        let name = v.get("name").and_then(Value::as_str).map(|s| s.to_string());
        let description = v
            .get("description")
            .or_else(|| v.get("desc"))
            .and_then(Value::as_str)
            .map(|s| s.to_string());
        let details = v.clone();
        let info = ItemInfo {
            name,
            unique_name: unique_name.to_string(),
            product_category,
            description,
            details,
        };
        index.entry(unique_name.to_string()).or_default().push(info);
    };

    // Warframes
    if let Some(raw) = read_cached("Warframes.json") {
        if let Ok(arr) = serde_json::from_str::<itemdata::warframe::Root>(&raw) {
            for item in arr {
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, Some("Suits".to_string()));
            }
        }
    }
    // Primary
    if let Some(raw) = read_cached("Primary.json") {
        if let Ok(arr) = serde_json::from_str::<itemdata::primary::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Secondary
    if let Some(raw) = read_cached("Secondary.json") {
        if let Ok(arr) = serde_json::from_str::<itemdata::secondary::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Melee
    if let Some(raw) = read_cached("Melee.json") {
        if let Ok(arr) = serde_json::from_str::<itemdata::melee::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Archwing suits
    if let Some(raw) = read_cached("Archwing.json") {
        if let Ok(arr) = serde_json::from_str::<itemdata::archwing::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Arch-guns
    if let Some(raw) = read_cached("Arch-Gun.json") {
        if let Ok(arr) = serde_json::from_str::<itemdata::arch_gun::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Arch-melee
    if let Some(raw) = read_cached("Arch-Melee.json") {
        if let Ok(arr) = serde_json::from_str::<itemdata::arch_melee::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Mods (covers Upgrades/RawUpgrades)
    if let Some(raw) = read_cached("Mods.json") {
        if let Ok(arr) = serde_json::from_str::<itemdata::mods::Root>(&raw) {
            for item in arr {
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                for pc in item.get_product_categories() {
                    push_info(&v, Some(pc));
                }
            }
        }
    }

    index
}

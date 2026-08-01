use std::collections::HashMap;
use std::fs;
use std::sync::LazyLock;

use serde::Serialize;

use crate::common::Patchlog;
use crate::traits::Item;
use crate::{
    ProductCategory, arch_gun, arch_melee, archwing, melee, mods, primary, secondary, warframe,
};

/// Typed detail payload for an indexed item. Implements [`Item`] by
/// delegating to the variant's payload via `enum_dispatch`.
#[enum_dispatch::enum_dispatch(Item)]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ItemDetails {
    Warframe(warframe::WarframeEntry),
    Primary(primary::Primary),
    Secondary(secondary::Secondary),
    Melee(melee::Melee),
    Archwing(archwing::Archwing),
    ArchGun(arch_gun::ArchGun),
    ArchMelee(arch_melee::ArchMelee),
    Mod(mods::ModEntry),
}

#[derive(Debug, Clone, Serialize)]
pub struct ItemInfo {
    pub name: Option<String>,
    pub unique_name: String,
    pub product_category: Option<String>,
    pub description: Option<String>,
    pub details: ItemDetails,
}

impl ItemInfo {
    fn new(details: ItemDetails, product_category: Option<String>) -> Self {
        Self {
            name: Some(details.name().to_string()),
            unique_name: details.unique_name().to_string(),
            product_category,
            description: details.description().map(|s| s.to_string()),
            details,
        }
    }
}

// Maps uniqueName/item_type -> all matching ItemInfo variants (multiple productCategory variants may exist)
static ITEM_INDEX: LazyLock<HashMap<String, Vec<ItemInfo>>> = LazyLock::new(build_item_index);

pub fn lookup_item_info(item_type: &str, category: Option<&str>) -> Option<ItemInfo> {
    let entries = ITEM_INDEX.get(item_type)?;
    if let Some(cat) = category.and_then(category_to_product_category)
        && let Some(found) = entries
            .iter()
            .find(|info| info.product_category.as_deref() == Some(cat))
    {
        return Some(found.clone());
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
        crate::item_data_fetch::cached_path(file)
            .ok()
            .and_then(|p| fs::read_to_string(p).ok())
    };

    let mut push_info = |details: ItemDetails, product_category: Option<String>| {
        let info = ItemInfo::new(details, product_category);
        index
            .entry(info.unique_name.clone())
            .or_default()
            .push(info);
    };

    // Warframes
    if let Some(raw) = read_cached("Warframes.json")
        && let Ok(arr) = serde_json::from_str::<warframe::Root>(&raw)
    {
        for item in arr {
            push_info(ItemDetails::Warframe(item), Some("Suits".to_string()));
        }
    }
    // Primary
    if let Some(raw) = read_cached("Primary.json")
        && let Ok(arr) = serde_json::from_str::<primary::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_str().to_string());
            push_info(ItemDetails::Primary(item), pc);
        }
    }
    // Secondary
    if let Some(raw) = read_cached("Secondary.json")
        && let Ok(arr) = serde_json::from_str::<secondary::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_str().to_string());
            push_info(ItemDetails::Secondary(item), pc);
        }
    }
    // Melee
    if let Some(raw) = read_cached("Melee.json")
        && let Ok(arr) = serde_json::from_str::<melee::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_str().to_string());
            push_info(ItemDetails::Melee(item), pc);
        }
    }
    // Archwing suits
    if let Some(raw) = read_cached("Archwing.json")
        && let Ok(arr) = serde_json::from_str::<archwing::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_str().to_string());
            push_info(ItemDetails::Archwing(item), pc);
        }
    }
    // Arch-guns
    if let Some(raw) = read_cached("Arch-Gun.json")
        && let Ok(arr) = serde_json::from_str::<arch_gun::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_str().to_string());
            push_info(ItemDetails::ArchGun(item), pc);
        }
    }
    // Arch-melee
    if let Some(raw) = read_cached("Arch-Melee.json")
        && let Ok(arr) = serde_json::from_str::<arch_melee::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_str().to_string());
            push_info(ItemDetails::ArchMelee(item), pc);
        }
    }
    // Mods (covers Upgrades/RawUpgrades)
    if let Some(raw) = read_cached("Mods.json")
        && let Ok(arr) = serde_json::from_str::<mods::Root>(&raw)
    {
        for item in arr {
            for pc in item.get_product_categories() {
                push_info(ItemDetails::Mod(item.clone()), Some(pc));
            }
        }
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn fixture(name: &str) -> String {
        std::fs::read_to_string(format!(
            "{}/testdata/itemdata/{}",
            env!("CARGO_MANIFEST_DIR"),
            name
        ))
        .unwrap()
    }

    /// The typed accessors must extract the same values the old code pulled
    /// from the serialized JSON via .get("uniqueName")/.get("name")/
    /// .get("description")/.get("tradable"), for every ItemDetails variant.
    #[test]
    fn accessors_match_legacy_json_extraction_for_all_variants() {
        let variants: Vec<ItemDetails> = vec![
            ItemDetails::Warframe(serde_json::from_str(&fixture("warframe_test.json")).unwrap()),
            ItemDetails::Primary(serde_json::from_str(&fixture("primary_test.json")).unwrap()),
            ItemDetails::Secondary(serde_json::from_str(&fixture("secondary_test.json")).unwrap()),
            ItemDetails::Melee(serde_json::from_str(&fixture("melee_test.json")).unwrap()),
            ItemDetails::Archwing(serde_json::from_str(&fixture("archwing_test.json")).unwrap()),
            ItemDetails::ArchGun(serde_json::from_str(&fixture("arch_gun_test.json")).unwrap()),
            ItemDetails::ArchMelee(serde_json::from_str(&fixture("arch_melee_test.json")).unwrap()),
            ItemDetails::Mod(serde_json::from_str(&fixture("mods_test_1.json")).unwrap()),
        ];

        for details in variants {
            let v = serde_json::to_value(&details).unwrap();
            assert_eq!(
                Some(details.unique_name()),
                v.get("uniqueName").and_then(Value::as_str),
                "uniqueName mismatch"
            );
            assert_eq!(
                Some(details.name()),
                v.get("name").and_then(Value::as_str),
                "name mismatch"
            );
            assert_eq!(
                details.description(),
                v.get("description").and_then(Value::as_str),
                "description mismatch"
            );
            assert_eq!(
                Some(details.tradable()),
                v.get("tradable").and_then(Value::as_bool),
                "tradable mismatch"
            );
        }
    }

    #[test]
    fn item_info_extracts_fields_from_details() {
        let item: warframe::WarframeEntry =
            serde_json::from_str(&fixture("warframe_test.json")).unwrap();
        let info = ItemInfo::new(ItemDetails::Warframe(item), Some("Suits".to_string()));
        assert_eq!(info.unique_name, "/Lotus/Powersuits/Priest/HarrowPrime");
        assert_eq!(info.name.as_deref(), Some("Harrow Prime"));
        assert!(info.description.is_some());
    }
}

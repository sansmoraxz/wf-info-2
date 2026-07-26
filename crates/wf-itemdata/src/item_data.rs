use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;

use serde::Serialize;

use crate::traits::Item;
use crate::{ProductCategory, arch_gun, arch_melee, archwing, melee, mods, primary, secondary, warframe};

/// Typed detail payload for an indexed item.
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

impl ItemDetails {
    fn as_item(&self) -> &dyn Item {
        match self {
            Self::Warframe(x) => x,
            Self::Primary(x) => x,
            Self::Secondary(x) => x,
            Self::Melee(x) => x,
            Self::Archwing(x) => x,
            Self::ArchGun(x) => x,
            Self::ArchMelee(x) => x,
            Self::Mod(x) => x,
        }
    }

    pub fn unique_name(&self) -> &str {
        self.as_item().unique_name()
    }

    pub fn name(&self) -> &str {
        self.as_item().name()
    }

    pub fn tradable(&self) -> bool {
        self.as_item().tradable()
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Warframe(warframe::WarframeEntry::Suits(x)) => x.detail.description.as_deref(),
            Self::Warframe(warframe::WarframeEntry::MechSuits(x)) => {
                x.detail.description.as_deref()
            }
            Self::Warframe(warframe::WarframeEntry::Helminth(x)) => x.detail.description.as_deref(),
            Self::Primary(x) => x.detail.description.as_deref(),
            Self::Secondary(x) => x.detail.description.as_deref(),
            Self::Melee(x) => x.detail.description.as_deref(),
            Self::Archwing(x) => x.detail.description.as_deref(),
            Self::ArchGun(x) => x.detail.description.as_deref(),
            Self::ArchMelee(x) => x.detail.description.as_deref(),
            Self::Mod(mods::ModEntry::Riven(x)) => x.detail.description.as_deref(),
            Self::Mod(mods::ModEntry::SetMember(x)) => x.detail.description.as_deref(),
            Self::Mod(mods::ModEntry::SetDefinition(x)) => x.detail.description.as_deref(),
            Self::Mod(mods::ModEntry::Regular(x)) => x.detail.description.as_deref(),
        }
    }
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
static ITEM_INDEX: OnceLock<HashMap<String, Vec<ItemInfo>>> = OnceLock::new();

pub fn lookup_item_info(item_type: &str, category: Option<&str>) -> Option<ItemInfo> {
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
    if let Some(raw) = read_cached("Warframes.json") {
        if let Ok(arr) = serde_json::from_str::<warframe::Root>(&raw) {
            for item in arr {
                push_info(ItemDetails::Warframe(item), Some("Suits".to_string()));
            }
        }
    }
    // Primary
    if let Some(raw) = read_cached("Primary.json") {
        if let Ok(arr) = serde_json::from_str::<primary::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                push_info(ItemDetails::Primary(item), pc);
            }
        }
    }
    // Secondary
    if let Some(raw) = read_cached("Secondary.json") {
        if let Ok(arr) = serde_json::from_str::<secondary::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                push_info(ItemDetails::Secondary(item), pc);
            }
        }
    }
    // Melee
    if let Some(raw) = read_cached("Melee.json") {
        if let Ok(arr) = serde_json::from_str::<melee::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                push_info(ItemDetails::Melee(item), pc);
            }
        }
    }
    // Archwing suits
    if let Some(raw) = read_cached("Archwing.json") {
        if let Ok(arr) = serde_json::from_str::<archwing::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                push_info(ItemDetails::Archwing(item), pc);
            }
        }
    }
    // Arch-guns
    if let Some(raw) = read_cached("Arch-Gun.json") {
        if let Ok(arr) = serde_json::from_str::<arch_gun::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                push_info(ItemDetails::ArchGun(item), pc);
            }
        }
    }
    // Arch-melee
    if let Some(raw) = read_cached("Arch-Melee.json") {
        if let Ok(arr) = serde_json::from_str::<arch_melee::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.as_str().to_string());
                push_info(ItemDetails::ArchMelee(item), pc);
            }
        }
    }
    // Mods (covers Upgrades/RawUpgrades)
    if let Some(raw) = read_cached("Mods.json") {
        if let Ok(arr) = serde_json::from_str::<mods::Root>(&raw) {
            for item in arr {
                for pc in item.get_product_categories() {
                    push_info(ItemDetails::Mod(item.clone()), Some(pc));
                }
            }
        }
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_details_serializes_as_inner_item() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/warframe_test.json"
        ));
        let item: warframe::WarframeEntry = serde_json::from_str(json_data).unwrap();
        let inner = serde_json::to_value(&item).unwrap();
        let wrapped = serde_json::to_value(ItemDetails::Warframe(item)).unwrap();
        assert_eq!(wrapped, inner);
    }

    #[test]
    fn item_info_extracts_fields_from_details() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/warframe_test.json"
        ));
        let item: warframe::WarframeEntry = serde_json::from_str(json_data).unwrap();
        let info = ItemInfo::new(ItemDetails::Warframe(item), Some("Suits".to_string()));
        assert_eq!(info.unique_name, "/Lotus/Powersuits/Priest/HarrowPrime");
        assert_eq!(info.name.as_deref(), Some("Harrow Prime"));
        assert!(info.description.is_some());
    }
}

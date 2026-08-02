use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use serde::Serialize;

use crate::common::Patchlog;
use crate::item_data_fetch::cached_path;
use crate::traits::Item;
use crate::{
    ProductCategory as _, arch_gun, arch_melee, archwing, melee, mods, primary, secondary, warframe,
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

/// An item's canonical `uniqueName` type path, e.g. `/Lotus/Powersuits/...`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    derive_more::Display,
    derive_more::From,
    derive_more::AsRef,
)]
#[display("{_0}")]
#[as_ref(str)]
pub struct UniqueName(String);

impl Borrow<str> for UniqueName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ItemInfo {
    pub name: Option<String>,
    pub unique_name: UniqueName,
    pub product_category: Option<String>,
    pub description: Option<String>,
    /// Shared: the same payload appears under multiple product categories,
    /// and consumers attach it to response envelopes without deep-cloning.
    pub details: Arc<ItemDetails>,
}

impl ItemInfo {
    fn new(details: Arc<ItemDetails>, product_category: Option<String>) -> Self {
        Self {
            name: Some(details.name().to_owned()),
            unique_name: UniqueName(details.unique_name().to_owned()),
            product_category,
            description: details.description().map(str::to_owned),
            details,
        }
    }
}

/// Index of all known items, keyed by uniqueName/item_type. Multiple
/// productCategory variants may exist per key. Built once from the cached
/// item-data files; own it at the composition root and share by reference.
pub struct ItemIndex(HashMap<UniqueName, Vec<ItemInfo>>);

impl Default for ItemIndex {
    fn default() -> Self {
        Self(build_item_index())
    }
}

impl ItemIndex {
    pub fn lookup(&self, item_type: &str, category: Option<&str>) -> Option<&ItemInfo> {
        let entries = self.0.get(item_type)?;
        if let Some(cat) = category.and_then(category_to_product_category)
            && let Some(found) = entries
                .iter()
                .find(|info| info.product_category.as_deref() == Some(cat))
        {
            return Some(found);
        }
        // fallback: first entry
        entries.first()
    }
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

fn build_item_index() -> HashMap<UniqueName, Vec<ItemInfo>> {
    let mut index: HashMap<UniqueName, Vec<ItemInfo>> = HashMap::new();

    let read_cached = |file: &str| -> Option<String> {
        cached_path(file)
            .ok()
            .and_then(|p| fs::read_to_string(p).ok())
    };

    let mut push_info = |details: Arc<ItemDetails>, product_category: Option<String>| {
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
            push_info(Arc::new(ItemDetails::Warframe(item)), Some("Suits".to_owned()));
        }
    }
    // Primary
    if let Some(raw) = read_cached("Primary.json")
        && let Ok(arr) = serde_json::from_str::<primary::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_ref().to_owned());
            push_info(Arc::new(ItemDetails::Primary(item)), pc);
        }
    }
    // Secondary
    if let Some(raw) = read_cached("Secondary.json")
        && let Ok(arr) = serde_json::from_str::<secondary::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_ref().to_owned());
            push_info(Arc::new(ItemDetails::Secondary(item)), pc);
        }
    }
    // Melee
    if let Some(raw) = read_cached("Melee.json")
        && let Ok(arr) = serde_json::from_str::<melee::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_ref().to_owned());
            push_info(Arc::new(ItemDetails::Melee(item)), pc);
        }
    }
    // Archwing suits
    if let Some(raw) = read_cached("Archwing.json")
        && let Ok(arr) = serde_json::from_str::<archwing::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_ref().to_owned());
            push_info(Arc::new(ItemDetails::Archwing(item)), pc);
        }
    }
    // Arch-guns
    if let Some(raw) = read_cached("Arch-Gun.json")
        && let Ok(arr) = serde_json::from_str::<arch_gun::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_ref().to_owned());
            push_info(Arc::new(ItemDetails::ArchGun(item)), pc);
        }
    }
    // Arch-melee
    if let Some(raw) = read_cached("Arch-Melee.json")
        && let Ok(arr) = serde_json::from_str::<arch_melee::Root>(&raw)
    {
        for item in arr {
            let pc = Some(item.product_category.as_ref().to_owned());
            push_info(Arc::new(ItemDetails::ArchMelee(item)), pc);
        }
    }
    // Mods (covers Upgrades/RawUpgrades)
    if let Some(raw) = read_cached("Mods.json")
        && let Ok(arr) = serde_json::from_str::<mods::Root>(&raw)
    {
        for item in arr {
            // One shared payload regardless of how many categories list it
            let categories = item.get_product_categories();
            let details = Arc::new(ItemDetails::Mod(item));
            for pc in categories {
                push_info(Arc::clone(&details), Some(pc));
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
        fs::read_to_string(format!(
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
        let info = ItemInfo::new(
            Arc::new(ItemDetails::Warframe(item)),
            Some("Suits".to_owned()),
        );
        assert_eq!(
            info.unique_name.as_ref(),
            "/Lotus/Powersuits/Priest/HarrowPrime"
        );
        assert_eq!(info.name.as_deref(), Some("Harrow Prime"));
        assert!(info.description.is_some());
    }
}

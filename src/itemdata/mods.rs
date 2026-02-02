//! Mod upgrade item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::common::{Drop, Introduced, LevelStat, Patchlog};
use crate::itemdata::enums::{Polarity, Rarity};
use crate::itemdata::traits::{Droppable, Item, WikiaLinked};
use crate::itemdata::ProductCategory;

pub type Root = Vec<Mod>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    // Core identity
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image_name: String,
    pub description: Option<String>,

    // Tradable
    pub tradable: bool,
    pub masterable: bool,

    // Mod-specific
    pub base_drain: Option<i64>,
    pub fusion_limit: Option<i64>,
    pub compat_name: Option<String>,
    #[serde(default)]
    pub polarity: Option<Polarity>,
    #[serde(default)]
    pub rarity: Option<Rarity>,
    pub transmutable: Option<bool>,
    pub is_augment: Option<bool>,
    #[serde(default)]
    pub is_prime: bool,
    pub is_utility: Option<bool>,
    pub is_exilus: Option<bool>,

    // Level stats
    #[serde(default)]
    pub level_stats: Vec<LevelStat>,

    // Mod sets
    pub mod_set: Option<String>,
    pub num_upgrades_in_set: Option<i64>,
    #[serde(default)]
    pub stats: Vec<String>,
    pub buff_set: Option<bool>,
    pub mod_set_values: Option<Vec<f64>>,

    // Riven-specific
    #[serde(default)]
    pub available_challenges: Vec<AvailableChallenge>,
    #[serde(default)]
    pub upgrade_entries: Vec<UpgradeEntry>,

    // Wikia
    pub wiki_available: Option<bool>,
    pub wikia_thumbnail: Option<String>,
    pub wikia_url: Option<String>,
    pub introduced: Option<Introduced>,
    pub release_date: Option<String>,
    pub exclude_from_codex: Option<bool>,

    // Droppable
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Mod {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Upgrades".to_string(), "RawUpgrades".to_string()]
    }
}

impl Item for Mod {
    fn unique_name(&self) -> &str {
        &self.unique_name
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn category(&self) -> &str {
        &self.category
    }
    fn type_field(&self) -> &str {
        &self.type_field
    }
    fn image_name(&self) -> Option<&str> {
        Some(&self.image_name)
    }
    fn tradable(&self) -> bool {
        self.tradable
    }
    fn masterable(&self) -> bool {
        self.masterable
    }
    fn patchlogs(&self) -> &[Patchlog] {
        &self.patchlogs
    }
}

impl Droppable for Mod {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl WikiaLinked for Mod {
    fn wiki_available(&self) -> Option<bool> {
        self.wiki_available
    }
    fn wikia_url(&self) -> Option<&str> {
        self.wikia_url.as_deref()
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        self.wikia_thumbnail.as_deref()
    }
    fn introduced(&self) -> Option<&Introduced> {
        self.introduced.as_ref()
    }
    fn release_date(&self) -> Option<&str> {
        self.release_date.as_deref()
    }
}

/// Riven challenge definition.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableChallenge {
    pub full_name: String,
    pub description: String,
    #[serde(default)]
    pub complications: Vec<Complication>,
}

/// Riven challenge complication modifier.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Complication {
    pub full_name: String,
    pub description: String,
    pub override_tag: Option<String>,
}

/// Riven upgrade entry.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeEntry {
    pub tag: String,
    pub prefix_tag: String,
    pub suffix_tag: String,
    #[serde(default)]
    pub upgrade_values: Vec<UpgradeValue>,
}

/// Riven upgrade value.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeValue {
    pub value: f64,
    pub loc_tag: Option<String>,
    pub reverse_value_symbol: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_raw_mod() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/mods_test_1.json"
        ));

        let m: Mod = from_str(json_data).unwrap();

        assert_eq!(
            m.unique_name,
            "/Lotus/Upgrades/Mods/Sets/Amar/AmarWarframeMod"
        );
        assert_eq!(
            m.mod_set,
            Some("/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod".into())
        );
    }

    #[test]
    fn test_deserialize_mod_set() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/mods_test_2.json"
        ));

        let m: Mod = from_str(json_data).unwrap();

        assert_eq!(m.unique_name, "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod");
        assert_eq!(m.num_upgrades_in_set, Some(3));
    }
}

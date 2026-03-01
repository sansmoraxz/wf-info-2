//! Mod upgrade item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, LevelStat, Patchlog};
use crate::itemdata::enums::{ModCategory, ModType, Polarity, Rarity};
use crate::itemdata::props::{ItemDetailProps, ItemIdentityProps, TradableProps, WikiaProps};
use crate::itemdata::traits::{Droppable, Item, WikiaLinked};

pub type Root = Vec<Mod>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: ModType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

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

    pub exclude_from_codex: Option<bool>,

    // Grouped props
    #[serde(flatten)]
    pub wikia: WikiaProps,

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

impl Mod {
    /// Get the computed mod category classification.
    ///
    /// Detection logic:
    /// - Has available_challenges (non-empty) → Riven
    /// - Has mod_set → SetMember
    /// - Has num_upgrades_in_set → SetDefinition
    /// - Otherwise → Regular
    pub fn mod_category(&self) -> ModCategory {
        if !self.available_challenges.is_empty() {
            ModCategory::Riven
        } else if let Some(mod_set) = &self.mod_set {
            ModCategory::SetMember {
                mod_set: mod_set.clone(),
            }
        } else if let Some(num) = self.num_upgrades_in_set {
            ModCategory::SetDefinition {
                num_upgrades_in_set: num,
            }
        } else {
            ModCategory::Regular
        }
    }

    /// Check if this is a Riven mod
    pub fn is_riven(&self) -> bool {
        self.mod_category().is_riven()
    }

    /// Check if this is part of a mod set (either member or definition)
    pub fn is_set(&self) -> bool {
        self.mod_category().is_set()
    }

    /// Check if this is a set member
    pub fn is_set_member(&self) -> bool {
        self.mod_category().is_set_member()
    }

    /// Check if this is a set definition
    pub fn is_set_definition(&self) -> bool {
        self.mod_category().is_set_definition()
    }

    /// Check if this is a regular mod
    pub fn is_regular(&self) -> bool {
        self.mod_category().is_regular()
    }
}

impl Item for Mod {
    fn unique_name(&self) -> &str {
        &self.identity.unique_name
    }
    fn name(&self) -> &str {
        &self.identity.name
    }
    fn category(&self) -> &str {
        &self.identity.category
    }
    fn type_field(&self) -> &str {
        self.type_field.as_str()
    }
    fn image_name(&self) -> Option<&str> {
        self.detail.image_name.as_deref()
    }
    fn tradable(&self) -> bool {
        self.trade.tradable
    }
    fn masterable(&self) -> bool {
        self.trade.masterable
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
        self.wikia.wiki_available
    }
    fn wikia_url(&self) -> Option<&str> {
        self.wikia.wikia_url.as_deref()
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        self.wikia.wikia_thumbnail.as_deref()
    }
    fn introduced(&self) -> Option<&crate::itemdata::common::Introduced> {
        self.wikia.introduced.as_ref()
    }
    fn release_date(&self) -> Option<&str> {
        self.wikia.release_date.as_deref()
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
            "/testdata/itemdata/mods_test_1.json"
        ));

        let m: Mod = from_str(json_data).unwrap();

        assert_eq!(
            m.identity.unique_name,
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
            "/testdata/itemdata/mods_test_2.json"
        ));

        let m: Mod = from_str(json_data).unwrap();

        assert_eq!(
            m.identity.unique_name,
            "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod"
        );
        assert_eq!(m.num_upgrades_in_set, Some(3));
    }

    #[test]
    fn test_mod_category_regular() {
        let m = Mod {
            level_stats: vec![],
            ..Default::default()
        };

        assert!(m.is_regular());
        assert!(!m.is_riven());
        assert!(!m.is_set());
        assert!(!m.is_set_member());
        assert!(!m.is_set_definition());
    }

    #[test]
    fn test_mod_category_set_member() {
        let m = Mod {
            mod_set: Some("/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod".to_string()),
            ..Default::default()
        };

        assert!(m.is_set());
        assert!(m.is_set_member());
        assert!(!m.is_set_definition());
        assert!(!m.is_riven());
        assert!(!m.is_regular());

        let cat = m.mod_category();
        assert_eq!(
            cat.mod_set(),
            Some("/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod")
        );
    }

    #[test]
    fn test_mod_category_set_definition() {
        let m = Mod {
            num_upgrades_in_set: Some(3),
            stats: vec!["Bonus 1".to_string(), "Bonus 2".to_string()],
            ..Default::default()
        };

        assert!(m.is_set());
        assert!(m.is_set_definition());
        assert!(!m.is_set_member());
        assert!(!m.is_riven());
        assert!(!m.is_regular());

        let cat = m.mod_category();
        assert_eq!(cat.num_upgrades_in_set(), Some(3));
    }

    #[test]
    fn test_mod_category_riven() {
        let m = Mod {
            available_challenges: vec![AvailableChallenge {
                full_name: "Test Challenge".to_string(),
                description: "Complete test".to_string(),
                complications: vec![],
            }],
            upgrade_entries: vec![],
            ..Default::default()
        };

        assert!(m.is_riven());
        assert!(!m.is_set());
        assert!(!m.is_regular());
    }
}

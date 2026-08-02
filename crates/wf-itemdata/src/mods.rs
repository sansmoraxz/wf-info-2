//! Mod upgrade item data.
//!
//! Four variants discriminated by field presence (tried in order):
//! - Riven : Has `availableChallenges` field
//! - SetMember : Has `modSet` field
//! - SetDefinition : Has `numUpgradesInSet` field
//! - Regular : Fallback (standard mods with level stats)

use serde::{Deserialize, Serialize};

use crate::ProductCategory;
use crate::common::{Drop, Introduced, LevelStat, Patchlog};
use crate::enums::{ModCategory, ModType, Polarity, Rarity};
use crate::props::{ItemDetailProps, ItemIdentityProps, TradableProps, WikiaProps};
use crate::traits::{Droppable, Item, WikiaLinked};

pub type Root = Vec<ModEntry>;

/// Mod entry. Uses `#[serde(untagged)]` — variants tried in order:
/// 1. RivenModData (requires `availableChallenges` field)
/// 2. SetMemberModData (requires `modSet` field)
/// 3. SetDefinitionModData (requires `numUpgradesInSet` field)
/// 4. RegularModData (fallback)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, derive_more::IsVariant)]
#[serde(untagged)]
pub enum ModEntry {
    Riven(RivenModData),
    SetMember(SetMemberModData),
    SetDefinition(SetDefinitionModData),
    Regular(RegularModData),
}

/// Regular mod (standard mods with level-based stats).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularModData {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: ModType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    // Mod stats (guaranteed for regular mods)
    pub base_drain: i64,
    pub fusion_limit: i64,
    pub polarity: Polarity,
    pub rarity: Rarity,

    #[serde(default)]
    pub is_prime: bool,
    pub compat_name: Option<String>,
    pub transmutable: Option<bool>,
    pub is_augment: Option<bool>,
    pub is_utility: Option<bool>,
    pub is_exilus: Option<bool>,
    pub exclude_from_codex: Option<bool>,

    #[serde(default)]
    pub level_stats: Vec<LevelStat>,

    #[serde(flatten)]
    pub wikia: WikiaProps,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

/// Set member mod (references a set definition via `modSet`).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMemberModData {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: ModType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    pub base_drain: i64,
    pub fusion_limit: i64,
    pub polarity: Polarity,
    pub rarity: Rarity,

    #[serde(default)]
    pub is_prime: bool,

    // Discriminator
    pub mod_set: String,

    pub compat_name: Option<String>,
    pub transmutable: Option<bool>,
    pub is_augment: Option<bool>,
    pub is_utility: Option<bool>,
    pub buff_set: Option<bool>,
    pub mod_set_values: Option<Vec<f64>>,

    #[serde(default)]
    pub level_stats: Vec<LevelStat>,

    #[serde(flatten)]
    pub wikia: WikiaProps,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

/// Set definition mod (describes set bonuses, `numUpgradesInSet` field).
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDefinitionModData {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: ModType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    #[serde(default)]
    pub is_prime: bool,

    // Discriminator
    pub num_upgrades_in_set: i64,
    #[serde(default)]
    pub stats: Vec<String>,
}

/// Riven mod with unveiling challenges and randomizable stat pools.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RivenModData {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: ModType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    pub base_drain: i64,
    pub fusion_limit: i64,
    pub polarity: Polarity,
    pub rarity: Rarity,

    #[serde(default)]
    pub is_prime: bool,
    pub transmutable: Option<bool>,

    // Discriminator — no #[serde(default)], must be present in JSON
    pub available_challenges: Vec<AvailableChallenge>,
    #[serde(default)]
    pub upgrade_entries: Vec<UpgradeEntry>,

    #[serde(flatten)]
    pub wikia: WikiaProps,

    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

// ── Trait implementations via match delegation ──

impl ProductCategory for ModEntry {
    fn get_product_categories(&self) -> Vec<String> {
        vec![
            "Upgrades".to_owned(),
            "RawUpgrades".to_owned(),
            "FocusUpgrades".to_owned(),
        ]
    }
}

impl ModEntry {
    /// Get the computed mod category classification.
    #[must_use]
    pub fn mod_category(&self) -> ModCategory {
        match self {
            Self::Riven(_) => ModCategory::Riven,
            Self::SetMember(m) => ModCategory::SetMember {
                mod_set: m.mod_set.clone(),
            },
            Self::SetDefinition(m) => ModCategory::SetDefinition {
                num_upgrades_in_set: m.num_upgrades_in_set,
            },
            Self::Regular(_) => ModCategory::Regular,
        }
    }

    #[must_use]
    pub fn is_set(&self) -> bool {
        self.is_set_member() || self.is_set_definition()
    }
}

impl Item for ModEntry {
    fn unique_name(&self) -> &str {
        match self {
            Self::Riven(m) => &m.identity.unique_name,
            Self::SetMember(m) => &m.identity.unique_name,
            Self::SetDefinition(m) => &m.identity.unique_name,
            Self::Regular(m) => &m.identity.unique_name,
        }
    }
    fn name(&self) -> &str {
        match self {
            Self::Riven(m) => &m.identity.name,
            Self::SetMember(m) => &m.identity.name,
            Self::SetDefinition(m) => &m.identity.name,
            Self::Regular(m) => &m.identity.name,
        }
    }
    fn category(&self) -> &str {
        match self {
            Self::Riven(m) => &m.identity.category,
            Self::SetMember(m) => &m.identity.category,
            Self::SetDefinition(m) => &m.identity.category,
            Self::Regular(m) => &m.identity.category,
        }
    }
    fn type_field(&self) -> &str {
        match self {
            Self::Riven(m) => m.type_field.as_ref(),
            Self::SetMember(m) => m.type_field.as_ref(),
            Self::SetDefinition(m) => m.type_field.as_ref(),
            Self::Regular(m) => m.type_field.as_ref(),
        }
    }
    fn image_name(&self) -> Option<&str> {
        match self {
            Self::Riven(m) => m.detail.image_name.as_deref(),
            Self::SetMember(m) => m.detail.image_name.as_deref(),
            Self::SetDefinition(m) => m.detail.image_name.as_deref(),
            Self::Regular(m) => m.detail.image_name.as_deref(),
        }
    }
    fn description(&self) -> Option<&str> {
        match self {
            Self::Riven(m) => m.detail.description.as_deref(),
            Self::SetMember(m) => m.detail.description.as_deref(),
            Self::SetDefinition(m) => m.detail.description.as_deref(),
            Self::Regular(m) => m.detail.description.as_deref(),
        }
    }
    fn tradable(&self) -> bool {
        match self {
            Self::Riven(m) => m.trade.tradable,
            Self::SetMember(m) => m.trade.tradable,
            Self::SetDefinition(m) => m.trade.tradable,
            Self::Regular(m) => m.trade.tradable,
        }
    }
    fn masterable(&self) -> bool {
        match self {
            Self::Riven(m) => m.trade.masterable,
            Self::SetMember(m) => m.trade.masterable,
            Self::SetDefinition(m) => m.trade.masterable,
            Self::Regular(m) => m.trade.masterable,
        }
    }
    fn patchlogs(&self) -> &[Patchlog] {
        match self {
            Self::Riven(m) => &m.patchlogs,
            Self::SetMember(m) => &m.patchlogs,
            Self::SetDefinition(_) => &[],
            Self::Regular(m) => &m.patchlogs,
        }
    }
}

impl Droppable for ModEntry {
    fn drops(&self) -> &[Drop] {
        match self {
            Self::SetMember(m) => &m.drops,
            Self::Regular(m) => &m.drops,
            Self::Riven(_) | Self::SetDefinition(_) => &[],
        }
    }
}

impl WikiaLinked for ModEntry {
    fn wiki_available(&self) -> Option<bool> {
        match self {
            Self::Riven(m) => m.wikia.wiki_available,
            Self::SetMember(m) => m.wikia.wiki_available,
            Self::SetDefinition(_) => None,
            Self::Regular(m) => m.wikia.wiki_available,
        }
    }
    fn wikia_url(&self) -> Option<&str> {
        match self {
            Self::Riven(m) => m.wikia.wikia_url.as_deref(),
            Self::SetMember(m) => m.wikia.wikia_url.as_deref(),
            Self::SetDefinition(_) => None,
            Self::Regular(m) => m.wikia.wikia_url.as_deref(),
        }
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        match self {
            Self::Riven(m) => m.wikia.wikia_thumbnail.as_deref(),
            Self::SetMember(m) => m.wikia.wikia_thumbnail.as_deref(),
            Self::SetDefinition(_) => None,
            Self::Regular(m) => m.wikia.wikia_thumbnail.as_deref(),
        }
    }
    fn introduced(&self) -> Option<&Introduced> {
        match self {
            Self::Riven(m) => m.wikia.introduced.as_ref(),
            Self::SetMember(m) => m.wikia.introduced.as_ref(),
            Self::SetDefinition(_) => None,
            Self::Regular(m) => m.wikia.introduced.as_ref(),
        }
    }
    fn release_date(&self) -> Option<&str> {
        match self {
            Self::Riven(m) => m.wikia.release_date.as_deref(),
            Self::SetMember(m) => m.wikia.release_date.as_deref(),
            Self::SetDefinition(_) => None,
            Self::Regular(m) => m.wikia.release_date.as_deref(),
        }
    }
}

/// Riven challenge definition.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableChallenge {
    pub full_name: String,
    pub description: String,
    #[serde(default)]
    pub complications: Vec<Complication>,
}

/// Riven challenge complication modifier.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    fn test_deserialize_set_member() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/mods_test_1.json"
        ));

        let rec: ModEntry = from_str(json_data).unwrap();

        match &rec {
            ModEntry::SetMember(m) => {
                assert_eq!(
                    m.identity.unique_name,
                    "/Lotus/Upgrades/Mods/Sets/Amar/AmarWarframeMod"
                );
                assert_eq!(m.mod_set, "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod");
                assert_eq!(m.rarity, Rarity::Uncommon);
                assert_eq!(m.is_augment, Some(true));
            }
            _ => panic!("Expected SetMember variant"),
        }
    }

    #[test]
    fn test_deserialize_set_definition() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/mods_test_2.json"
        ));

        let rec: ModEntry = from_str(json_data).unwrap();

        match &rec {
            ModEntry::SetDefinition(m) => {
                assert_eq!(
                    m.identity.unique_name,
                    "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod"
                );
                assert_eq!(m.num_upgrades_in_set, 3);
                assert_eq!(m.stats.len(), 3);
            }
            _ => panic!("Expected SetDefinition variant"),
        }
    }

    #[test]
    fn test_mod_category_from_variant() {
        let regular = ModEntry::Regular(RegularModData::default());
        assert!(regular.is_regular());
        assert!(!regular.is_riven());
        assert!(!regular.is_set());

        let set_member = ModEntry::SetMember(SetMemberModData {
            mod_set: "/Lotus/Test".to_owned(),
            ..Default::default()
        });
        assert!(set_member.is_set());
        assert!(set_member.is_set_member());
        assert!(!set_member.is_set_definition());

        let cat = set_member.mod_category();
        assert_eq!(cat.mod_set(), Some("/Lotus/Test"));

        let set_def = ModEntry::SetDefinition(SetDefinitionModData {
            num_upgrades_in_set: 3,
            ..Default::default()
        });
        assert!(set_def.is_set());
        assert!(set_def.is_set_definition());
        assert!(!set_def.is_set_member());

        let cat = set_def.mod_category();
        assert_eq!(cat.num_upgrades_in_set(), Some(3));

        let riven = ModEntry::Riven(RivenModData {
            available_challenges: vec![AvailableChallenge {
                full_name: "Test".to_owned(),
                description: "Test".to_owned(),
                complications: vec![],
            }],
            ..Default::default()
        });
        assert!(riven.is_riven());
        assert!(!riven.is_set());
        assert!(!riven.is_regular());
    }

    #[test]
    fn test_deserialize_riven() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/mods_test_3.json"
        ));

        let rec: ModEntry = from_str(json_data).unwrap();

        match &rec {
            ModEntry::Riven(m) => {
                assert_eq!(m.identity.name, "Archgun Riven Mod");
                assert_eq!(m.polarity, Polarity::Universal);
                assert_eq!(m.rarity, Rarity::Common);
                assert_eq!(m.base_drain, 0);
                assert_eq!(m.available_challenges.len(), 1);
            }
            _ => panic!("Expected Riven variant"),
        }
    }

    #[test]
    fn test_deserialize_regular_mod() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/mods_test_4.json"
        ));

        let rec: ModEntry = from_str(json_data).unwrap();

        match &rec {
            ModEntry::Regular(m) => {
                assert_eq!(m.identity.name, "Serration");
                assert_eq!(m.polarity, Polarity::Madurai);
                assert_eq!(m.rarity, Rarity::Uncommon);
                assert_eq!(m.base_drain, 2);
                assert_eq!(m.fusion_limit, 3);
                assert!(m.trade.tradable);
            }
            _ => panic!("Expected Regular variant"),
        }
    }
}

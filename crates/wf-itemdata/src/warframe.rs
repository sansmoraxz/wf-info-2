//! Warframe character suit item data.
//!
//! Three variants discriminated by `productCategory`:
//! - Suits : Standard warframes with full data
//! - MechSuits : Necramechs (eg: Bonewidow, Voidrig)
//! - Helminth : No productCategory, minimal fields

use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats, serde_as};

use crate::ProductCategory;
use crate::common::{Ability, Drop, Introduced, Patchlog};
use crate::components::Component;
use crate::enums::{Polarity, Sex, Slot, WarframeType};
use crate::props::{
    BuildableProps, CharacterStats, ItemDetailProps, ItemIdentityProps, PrimeProps, TradableProps,
};
use crate::traits::{
    Buildable, Character, Droppable, Equippable, HasAbilities, Item, Prime, WikiaLinked,
};

pub type Root = Vec<WarframeEntry>;

/// Warframe entry. Uses `#[serde(untagged)]` — variants tried in order:
/// 1. WarframeData (requires `sex` field → only matches Suits)
/// 2. NecramechData (requires `description` → matches MechSuits)
/// 3. HelminthData (fallback with minimal fields)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WarframeEntry {
    Suits(WarframeData),
    MechSuits(NecramechData),
    Helminth(HelminthData),
}

/// Standard warframe (113 entries, productCategory = "Suits").
#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarframeData {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: WarframeType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,
    pub mastery_req: i64,
    pub product_category: String,

    // Warframe-specific (guaranteed for Suits — discriminating field: sex)
    pub sex: Sex,
    #[serde(default)]
    pub abilities: Vec<Ability>,
    #[serde_as(as = "Option<OneOrMany<_, formats::PreferOne>>")]
    pub aura: Option<Vec<String>>,
    pub passive_description: String,
    pub color: i64,
    pub conclave: bool,

    // Wikia (guaranteed non-optional for Suits)
    pub introduced: Introduced,
    pub release_date: String,
    pub wiki_available: bool,
    pub wikia_url: String,

    #[serde(default)]
    pub polarities: Vec<Polarity>,
    #[serde(default)]
    pub exalted: Vec<String>,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,

    // Grouped props
    #[serde(flatten)]
    pub stats: CharacterStats,
    #[serde(flatten)]
    pub build: BuildableProps,
    #[serde(flatten)]
    pub prime: PrimeProps,
}

/// Necramech (2 entries, productCategory = "MechSuits").
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NecramechData {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: WarframeType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,
    pub mastery_req: i64,
    pub product_category: String,

    #[serde(default)]
    pub abilities: Vec<Ability>,

    #[serde(flatten)]
    pub build: BuildableProps,

    #[serde(default)]
    pub exalted: Vec<String>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,

    // Grouped props
    #[serde(flatten)]
    pub stats: CharacterStats,
    #[serde(flatten)]
    pub prime: PrimeProps,
}

/// Helminth (1 entry, no productCategory, all stats = 0, 13 abilities).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelminthData {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: WarframeType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    #[serde(default)]
    pub abilities: Vec<Ability>,
    #[serde(default)]
    pub drops: Vec<Drop>,

    // Grouped props
    #[serde(flatten)]
    pub stats: CharacterStats,
    #[serde(flatten)]
    pub prime: PrimeProps,
}

// ── Trait implementations via match delegation ──

impl ProductCategory for WarframeEntry {
    fn get_product_categories(&self) -> Vec<String> {
        match self {
            WarframeEntry::Suits(_) => vec!["Suits".to_string()],
            WarframeEntry::MechSuits(_) => vec!["MechSuits".to_string()],
            WarframeEntry::Helminth(_) => vec![],
        }
    }
}

impl Item for WarframeEntry {
    fn unique_name(&self) -> &str {
        match self {
            WarframeEntry::Suits(w) => &w.identity.unique_name,
            WarframeEntry::MechSuits(w) => &w.identity.unique_name,
            WarframeEntry::Helminth(w) => &w.identity.unique_name,
        }
    }
    fn name(&self) -> &str {
        match self {
            WarframeEntry::Suits(w) => &w.identity.name,
            WarframeEntry::MechSuits(w) => &w.identity.name,
            WarframeEntry::Helminth(w) => &w.identity.name,
        }
    }
    fn category(&self) -> &str {
        match self {
            WarframeEntry::Suits(w) => &w.identity.category,
            WarframeEntry::MechSuits(w) => &w.identity.category,
            WarframeEntry::Helminth(w) => &w.identity.category,
        }
    }
    fn type_field(&self) -> &str {
        match self {
            WarframeEntry::Suits(w) => w.type_field.as_str(),
            WarframeEntry::MechSuits(w) => w.type_field.as_str(),
            WarframeEntry::Helminth(w) => w.type_field.as_str(),
        }
    }
    fn image_name(&self) -> Option<&str> {
        match self {
            WarframeEntry::Suits(w) => w.detail.image_name.as_deref(),
            WarframeEntry::MechSuits(w) => w.detail.image_name.as_deref(),
            WarframeEntry::Helminth(w) => w.detail.image_name.as_deref(),
        }
    }
    fn tradable(&self) -> bool {
        match self {
            WarframeEntry::Suits(w) => w.trade.tradable,
            WarframeEntry::MechSuits(w) => w.trade.tradable,
            WarframeEntry::Helminth(w) => w.trade.tradable,
        }
    }
    fn masterable(&self) -> bool {
        match self {
            WarframeEntry::Suits(w) => w.trade.masterable,
            WarframeEntry::MechSuits(w) => w.trade.masterable,
            WarframeEntry::Helminth(w) => w.trade.masterable,
        }
    }
    fn patchlogs(&self) -> &[Patchlog] {
        match self {
            WarframeEntry::Suits(w) => &w.patchlogs,
            WarframeEntry::MechSuits(w) => &w.patchlogs,
            WarframeEntry::Helminth(_) => &[],
        }
    }
}

impl Droppable for WarframeEntry {
    fn drops(&self) -> &[Drop] {
        match self {
            WarframeEntry::Suits(w) => &w.drops,
            WarframeEntry::Helminth(w) => &w.drops,
            WarframeEntry::MechSuits(_) => &[],
        }
    }
}

impl Buildable for WarframeEntry {
    fn build_price(&self) -> Option<i64> {
        match self {
            WarframeEntry::Suits(w) => w.build.build_price,
            WarframeEntry::MechSuits(w) => w.build.build_price,
            WarframeEntry::Helminth(_) => None,
        }
    }
    fn build_quantity(&self) -> Option<i64> {
        match self {
            WarframeEntry::Suits(w) => w.build.build_quantity,
            WarframeEntry::MechSuits(w) => w.build.build_quantity,
            WarframeEntry::Helminth(_) => None,
        }
    }
    fn build_time(&self) -> Option<i64> {
        match self {
            WarframeEntry::Suits(w) => w.build.build_time,
            WarframeEntry::MechSuits(w) => w.build.build_time,
            WarframeEntry::Helminth(_) => None,
        }
    }
    fn skip_build_time_price(&self) -> Option<i64> {
        match self {
            WarframeEntry::Suits(w) => w.build.skip_build_time_price,
            WarframeEntry::MechSuits(w) => w.build.skip_build_time_price,
            WarframeEntry::Helminth(_) => None,
        }
    }
    fn consume_on_build(&self) -> Option<bool> {
        match self {
            WarframeEntry::Suits(w) => w.build.consume_on_build,
            WarframeEntry::MechSuits(w) => w.build.consume_on_build,
            WarframeEntry::Helminth(_) => None,
        }
    }
    fn mastery_req(&self) -> Option<i64> {
        match self {
            WarframeEntry::Suits(w) => Some(w.mastery_req),
            WarframeEntry::MechSuits(w) => Some(w.mastery_req),
            WarframeEntry::Helminth(_) => None,
        }
    }
    fn market_cost(&self) -> Option<i64> {
        match self {
            WarframeEntry::Suits(w) => w.build.market_cost,
            _ => None,
        }
    }
    fn bp_cost(&self) -> Option<i64> {
        match self {
            WarframeEntry::Suits(w) => w.build.bp_cost,
            _ => None,
        }
    }
    fn components(&self) -> &[Component] {
        match self {
            WarframeEntry::Suits(w) => &w.build.components,
            WarframeEntry::MechSuits(w) => &w.build.components,
            WarframeEntry::Helminth(_) => &[],
        }
    }
}

impl Prime for WarframeEntry {
    fn is_prime(&self) -> bool {
        match self {
            WarframeEntry::Suits(w) => w.prime.is_prime,
            WarframeEntry::MechSuits(w) => w.prime.is_prime,
            WarframeEntry::Helminth(w) => w.prime.is_prime,
        }
    }
    fn vaulted(&self) -> Option<bool> {
        match self {
            WarframeEntry::Suits(w) => w.prime.vaulted,
            _ => None,
        }
    }
    fn vault_date(&self) -> Option<&str> {
        match self {
            WarframeEntry::Suits(w) => w.prime.vault_date.as_deref(),
            _ => None,
        }
    }
    fn estimated_vault_date(&self) -> Option<&str> {
        match self {
            WarframeEntry::Suits(w) => w.prime.estimated_vault_date.as_deref(),
            _ => None,
        }
    }
}

impl WikiaLinked for WarframeEntry {
    fn wiki_available(&self) -> Option<bool> {
        match self {
            WarframeEntry::Suits(w) => Some(w.wiki_available),
            _ => None,
        }
    }
    fn wikia_url(&self) -> Option<&str> {
        match self {
            WarframeEntry::Suits(w) => Some(&w.wikia_url),
            _ => None,
        }
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        None
    }
    fn introduced(&self) -> Option<&Introduced> {
        match self {
            WarframeEntry::Suits(w) => Some(&w.introduced),
            _ => None,
        }
    }
    fn release_date(&self) -> Option<&str> {
        match self {
            WarframeEntry::Suits(w) => Some(&w.release_date),
            _ => None,
        }
    }
}

impl Character for WarframeEntry {
    fn health(&self) -> i64 {
        match self {
            WarframeEntry::Suits(w) => w.stats.health,
            WarframeEntry::MechSuits(w) => w.stats.health,
            WarframeEntry::Helminth(w) => w.stats.health,
        }
    }
    fn shield(&self) -> i64 {
        match self {
            WarframeEntry::Suits(w) => w.stats.shield,
            WarframeEntry::MechSuits(w) => w.stats.shield,
            WarframeEntry::Helminth(w) => w.stats.shield,
        }
    }
    fn armor(&self) -> i64 {
        match self {
            WarframeEntry::Suits(w) => w.stats.armor,
            WarframeEntry::MechSuits(w) => w.stats.armor,
            WarframeEntry::Helminth(w) => w.stats.armor,
        }
    }
    fn power(&self) -> i64 {
        match self {
            WarframeEntry::Suits(w) => w.stats.power,
            WarframeEntry::MechSuits(w) => w.stats.power,
            WarframeEntry::Helminth(w) => w.stats.power,
        }
    }
    fn stamina(&self) -> i64 {
        match self {
            WarframeEntry::Suits(w) => w.stats.stamina,
            WarframeEntry::MechSuits(w) => w.stats.stamina,
            WarframeEntry::Helminth(w) => w.stats.stamina,
        }
    }
    fn sprint_speed(&self) -> Option<f64> {
        match self {
            WarframeEntry::Suits(w) => w.stats.sprint_speed,
            WarframeEntry::MechSuits(w) => w.stats.sprint_speed,
            WarframeEntry::Helminth(_) => None,
        }
    }
}

impl HasAbilities for WarframeEntry {
    fn abilities(&self) -> &[Ability] {
        match self {
            WarframeEntry::Suits(w) => &w.abilities,
            WarframeEntry::MechSuits(w) => &w.abilities,
            WarframeEntry::Helminth(w) => &w.abilities,
        }
    }
}

impl Equippable for WarframeEntry {
    fn polarities(&self) -> &[Polarity] {
        match self {
            WarframeEntry::Suits(w) => &w.polarities,
            _ => &[],
        }
    }
    fn slot(&self) -> Option<&Slot> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_warframe() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/warframe_test.json"
        ));

        let rec: WarframeEntry = from_str(json_data).unwrap();

        match &rec {
            WarframeEntry::Suits(w) => {
                assert_eq!(
                    w.identity.unique_name,
                    "/Lotus/Powersuits/Priest/HarrowPrime"
                );
                assert_eq!(w.identity.name, "Harrow Prime");
                assert_eq!(w.identity.category, "Warframes");
                assert_eq!(w.type_field, WarframeType::Warframe);
                assert_eq!(w.sex, Sex::Male);
                assert!(!w.trade.tradable);
                assert!(w.trade.masterable);
                assert_eq!(w.product_category, "Suits");

                // Character stats
                assert_eq!(w.stats.health, 270);
                assert_eq!(w.stats.shield, 640);
                assert_eq!(w.stats.armor, 185);
                assert_eq!(w.stats.power, 140);

                // Abilities
                assert_eq!(w.abilities.len(), 4);
                assert_eq!(w.abilities[0].name, "Condemn");

                // Buildable
                assert_eq!(w.build.build_price, Some(25000));
                assert_eq!(w.build.components.len(), 5);

                // Prime
                assert!(w.prime.is_prime);
            }
            _ => panic!("Expected Suits variant"),
        }
    }

    #[test]
    fn test_deserialize_warframe_prime() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/warframe_test_2.json"
        ));

        let rec: WarframeEntry = from_str(json_data).unwrap();

        match &rec {
            WarframeEntry::Suits(w) => {
                assert_eq!(w.identity.name, "Ash Prime");
                assert_eq!(w.product_category, "Suits");
                assert_eq!(w.sex, Sex::Male);
                assert!(w.prime.is_prime);
                assert_eq!(w.prime.vaulted, Some(true));
                assert_eq!(w.stats.health, 455);
                assert_eq!(w.stats.shield, 365);
                assert_eq!(w.stats.armor, 185);
                assert_eq!(w.abilities.len(), 4);
                assert_eq!(w.build.components.len(), 5);
            }
            _ => panic!("Expected Suits variant"),
        }
    }

    #[test]
    fn test_deserialize_necramech() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/warframe_test_3.json"
        ));

        let rec: WarframeEntry = from_str(json_data).unwrap();

        match &rec {
            WarframeEntry::MechSuits(w) => {
                assert_eq!(w.identity.name, "Bonewidow");
                assert_eq!(w.product_category, "MechSuits");
                assert!(!w.prime.is_prime);
                assert_eq!(w.stats.health, 1880);
                assert_eq!(w.stats.shield, 430);
                assert_eq!(w.stats.armor, 480);
                assert_eq!(w.stats.power, 175);
                assert_eq!(w.abilities.len(), 4);
                assert_eq!(w.build.build_price, Some(25000));
                assert_eq!(w.build.components.len(), 5);
            }
            _ => panic!("Expected MechSuits variant"),
        }
    }
}

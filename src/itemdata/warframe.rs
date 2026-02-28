//! Warframe character suit item data.
//!
//! Three variants discriminated by `productCategory`:
//! - Suits : Standard warframes with full data
//! - MechSuits : Necramechs (eg: Bonewidow, Voidrig)
//! - Helminth : No productCategory, minimal fields

use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats, serde_as};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Ability, Drop, Introduced, Patchlog};
use crate::itemdata::components::Component;
use crate::itemdata::enums::{Polarity, Sex, Slot};
use crate::itemdata::traits::{
    Buildable, Character, Droppable, Equippable, HasAbilities, Item, Prime, WikiaLinked,
};

pub type Root = Vec<Warframe>;

/// Warframe entry. Uses `#[serde(untagged)]` — variants tried in order:
/// 1. WarframeData (requires `sex` field → only matches Suits)
/// 2. NecramechData (requires `description` → matches MechSuits)
/// 3. HelminthData (fallback with minimal fields)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Warframe {
    Suits(WarframeData),
    MechSuits(NecramechData),
    Helminth(HelminthData),
}

/// Standard warframe (113 entries, productCategory = "Suits").
#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarframeData {
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image_name: String,
    pub description: String,
    pub tradable: bool,
    pub masterable: bool,
    pub mastery_req: i64,
    pub product_category: String,

    // Character stats
    pub health: i64,
    pub shield: i64,
    pub armor: i64,
    pub power: i64,
    pub stamina: i64,
    pub sprint: f64,
    pub sprint_speed: f64,

    // Warframe-specific (guaranteed for Suits — discriminating field: sex)
    pub sex: Sex,
    #[serde(default)]
    pub abilities: Vec<Ability>,
    #[serde_as(as = "Option<OneOrMany<_, formats::PreferOne>>")]
    pub aura: Option<Vec<String>>,
    pub passive_description: String,
    pub color: i64,
    pub conclave: bool,
    pub introduced: Introduced,
    pub release_date: String,
    pub wiki_available: bool,
    pub wikia_url: String,
    #[serde(default)]
    pub polarities: Vec<Polarity>,
    #[serde(default)]
    pub exalted: Vec<String>,

    // Buildable (optional — most but not all Suits have these)
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub skip_build_time_price: Option<i64>,
    pub consume_on_build: Option<bool>,
    pub market_cost: Option<i64>,
    pub bp_cost: Option<i64>,
    #[serde(default)]
    pub components: Vec<Component>,

    // Prime/vault
    #[serde(default)]
    pub is_prime: bool,
    pub vaulted: Option<bool>,
    pub vault_date: Option<String>,
    pub estimated_vault_date: Option<String>,

    // Droppable
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

/// Necramech (2 entries, productCategory = "MechSuits").
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NecramechData {
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image_name: String,
    pub description: String,
    pub tradable: bool,
    pub masterable: bool,
    pub mastery_req: i64,
    pub product_category: String,

    // Character stats
    pub health: i64,
    pub shield: i64,
    pub armor: i64,
    pub power: i64,
    pub stamina: i64,
    pub sprint_speed: f64,

    #[serde(default)]
    pub abilities: Vec<Ability>,

    // Buildable (guaranteed for MechSuits)
    pub build_price: i64,
    pub build_quantity: i64,
    pub build_time: i64,
    pub skip_build_time_price: i64,
    pub consume_on_build: bool,
    #[serde(default)]
    pub components: Vec<Component>,

    #[serde(default)]
    pub is_prime: bool,
    #[serde(default)]
    pub exalted: Vec<String>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

/// Helminth (1 entry, no productCategory, all stats = 0, 13 abilities).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelminthData {
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub tradable: bool,
    pub masterable: bool,

    pub health: i64,
    pub shield: i64,
    pub armor: i64,
    pub power: i64,
    pub stamina: i64,

    #[serde(default)]
    pub is_prime: bool,
    #[serde(default)]
    pub abilities: Vec<Ability>,
    #[serde(default)]
    pub drops: Vec<Drop>,
}

// ── Trait implementations via match delegation ──

impl ProductCategory for Warframe {
    fn get_product_categories(&self) -> Vec<String> {
        match self {
            Warframe::Suits(_) => vec!["Suits".to_string()],
            Warframe::MechSuits(_) => vec!["MechSuits".to_string()],
            Warframe::Helminth(_) => vec![],
        }
    }
}

impl Item for Warframe {
    fn unique_name(&self) -> &str {
        match self {
            Warframe::Suits(w) => &w.unique_name,
            Warframe::MechSuits(w) => &w.unique_name,
            Warframe::Helminth(w) => &w.unique_name,
        }
    }
    fn name(&self) -> &str {
        match self {
            Warframe::Suits(w) => &w.name,
            Warframe::MechSuits(w) => &w.name,
            Warframe::Helminth(w) => &w.name,
        }
    }
    fn category(&self) -> &str {
        match self {
            Warframe::Suits(w) => &w.category,
            Warframe::MechSuits(w) => &w.category,
            Warframe::Helminth(w) => &w.category,
        }
    }
    fn type_field(&self) -> &str {
        match self {
            Warframe::Suits(w) => &w.type_field,
            Warframe::MechSuits(w) => &w.type_field,
            Warframe::Helminth(w) => &w.type_field,
        }
    }
    fn image_name(&self) -> Option<&str> {
        match self {
            Warframe::Suits(w) => Some(&w.image_name),
            Warframe::MechSuits(w) => Some(&w.image_name),
            Warframe::Helminth(_) => None,
        }
    }
    fn tradable(&self) -> bool {
        match self {
            Warframe::Suits(w) => w.tradable,
            Warframe::MechSuits(w) => w.tradable,
            Warframe::Helminth(w) => w.tradable,
        }
    }
    fn masterable(&self) -> bool {
        match self {
            Warframe::Suits(w) => w.masterable,
            Warframe::MechSuits(w) => w.masterable,
            Warframe::Helminth(w) => w.masterable,
        }
    }
    fn patchlogs(&self) -> &[Patchlog] {
        match self {
            Warframe::Suits(w) => &w.patchlogs,
            Warframe::MechSuits(w) => &w.patchlogs,
            Warframe::Helminth(_) => &[],
        }
    }
}

impl Droppable for Warframe {
    fn drops(&self) -> &[Drop] {
        match self {
            Warframe::Suits(w) => &w.drops,
            Warframe::Helminth(w) => &w.drops,
            Warframe::MechSuits(_) => &[],
        }
    }
}

impl Buildable for Warframe {
    fn build_price(&self) -> Option<i64> {
        match self {
            Warframe::Suits(w) => w.build_price,
            Warframe::MechSuits(w) => Some(w.build_price),
            Warframe::Helminth(_) => None,
        }
    }
    fn build_quantity(&self) -> Option<i64> {
        match self {
            Warframe::Suits(w) => w.build_quantity,
            Warframe::MechSuits(w) => Some(w.build_quantity),
            Warframe::Helminth(_) => None,
        }
    }
    fn build_time(&self) -> Option<i64> {
        match self {
            Warframe::Suits(w) => w.build_time,
            Warframe::MechSuits(w) => Some(w.build_time),
            Warframe::Helminth(_) => None,
        }
    }
    fn skip_build_time_price(&self) -> Option<i64> {
        match self {
            Warframe::Suits(w) => w.skip_build_time_price,
            Warframe::MechSuits(w) => Some(w.skip_build_time_price),
            Warframe::Helminth(_) => None,
        }
    }
    fn consume_on_build(&self) -> Option<bool> {
        match self {
            Warframe::Suits(w) => w.consume_on_build,
            Warframe::MechSuits(w) => Some(w.consume_on_build),
            Warframe::Helminth(_) => None,
        }
    }
    fn mastery_req(&self) -> Option<i64> {
        match self {
            Warframe::Suits(w) => Some(w.mastery_req),
            Warframe::MechSuits(w) => Some(w.mastery_req),
            Warframe::Helminth(_) => None,
        }
    }
    fn market_cost(&self) -> Option<i64> {
        match self {
            Warframe::Suits(w) => w.market_cost,
            _ => None,
        }
    }
    fn bp_cost(&self) -> Option<i64> {
        match self {
            Warframe::Suits(w) => w.bp_cost,
            _ => None,
        }
    }
    fn components(&self) -> &[Component] {
        match self {
            Warframe::Suits(w) => &w.components,
            Warframe::MechSuits(w) => &w.components,
            Warframe::Helminth(_) => &[],
        }
    }
}

impl Prime for Warframe {
    fn is_prime(&self) -> bool {
        match self {
            Warframe::Suits(w) => w.is_prime,
            Warframe::MechSuits(w) => w.is_prime,
            Warframe::Helminth(w) => w.is_prime,
        }
    }
    fn vaulted(&self) -> Option<bool> {
        match self {
            Warframe::Suits(w) => w.vaulted,
            _ => None,
        }
    }
    fn vault_date(&self) -> Option<&str> {
        match self {
            Warframe::Suits(w) => w.vault_date.as_deref(),
            _ => None,
        }
    }
    fn estimated_vault_date(&self) -> Option<&str> {
        match self {
            Warframe::Suits(w) => w.estimated_vault_date.as_deref(),
            _ => None,
        }
    }
}

impl WikiaLinked for Warframe {
    fn wiki_available(&self) -> Option<bool> {
        match self {
            Warframe::Suits(w) => Some(w.wiki_available),
            _ => None,
        }
    }
    fn wikia_url(&self) -> Option<&str> {
        match self {
            Warframe::Suits(w) => Some(&w.wikia_url),
            _ => None,
        }
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        None
    }
    fn introduced(&self) -> Option<&Introduced> {
        match self {
            Warframe::Suits(w) => Some(&w.introduced),
            _ => None,
        }
    }
    fn release_date(&self) -> Option<&str> {
        match self {
            Warframe::Suits(w) => Some(&w.release_date),
            _ => None,
        }
    }
}

impl Character for Warframe {
    fn health(&self) -> i64 {
        match self {
            Warframe::Suits(w) => w.health,
            Warframe::MechSuits(w) => w.health,
            Warframe::Helminth(w) => w.health,
        }
    }
    fn shield(&self) -> i64 {
        match self {
            Warframe::Suits(w) => w.shield,
            Warframe::MechSuits(w) => w.shield,
            Warframe::Helminth(w) => w.shield,
        }
    }
    fn armor(&self) -> i64 {
        match self {
            Warframe::Suits(w) => w.armor,
            Warframe::MechSuits(w) => w.armor,
            Warframe::Helminth(w) => w.armor,
        }
    }
    fn power(&self) -> i64 {
        match self {
            Warframe::Suits(w) => w.power,
            Warframe::MechSuits(w) => w.power,
            Warframe::Helminth(w) => w.power,
        }
    }
    fn stamina(&self) -> i64 {
        match self {
            Warframe::Suits(w) => w.stamina,
            Warframe::MechSuits(w) => w.stamina,
            Warframe::Helminth(w) => w.stamina,
        }
    }
    fn sprint_speed(&self) -> Option<f64> {
        match self {
            Warframe::Suits(w) => Some(w.sprint_speed),
            Warframe::MechSuits(w) => Some(w.sprint_speed),
            Warframe::Helminth(_) => None,
        }
    }
}

impl HasAbilities for Warframe {
    fn abilities(&self) -> &[Ability] {
        match self {
            Warframe::Suits(w) => &w.abilities,
            Warframe::MechSuits(w) => &w.abilities,
            Warframe::Helminth(w) => &w.abilities,
        }
    }
}

impl Equippable for Warframe {
    fn polarities(&self) -> &[Polarity] {
        match self {
            Warframe::Suits(w) => &w.polarities,
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

        let rec: Warframe = from_str(json_data).unwrap();

        match &rec {
            Warframe::Suits(w) => {
                assert_eq!(w.unique_name, "/Lotus/Powersuits/Priest/HarrowPrime");
                assert_eq!(w.sex, Sex::Male);
            }
            _ => panic!("Expected Suits variant"),
        }
    }
}

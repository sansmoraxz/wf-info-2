//! Companion pet item data.
//!
//! Pets are discriminated by `productCategory`:
//! - KubrowPets: Combat companions (Kavats, Kubrows, Vulpaphylas, etc.)
//! - Pistols: Crafting components (mutagens, antigens, cores, gyros)
//! - SpecialItems: Warframe companions (Venari, Venari Prime)

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, Introduced, Patchlog};
use crate::itemdata::components::Component;
use crate::itemdata::enums::Polarity;
use crate::itemdata::props::CharacterStats;
use crate::itemdata::traits::{Buildable, Droppable, Equippable, Item, WikiaLinked};

pub type Root = Vec<PetEntry>;

/// Pet entry, discriminated by `productCategory`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "productCategory")]
pub enum PetEntry {
    /// Combat companion pets (Kavats, Kubrows, Vulpaphylas, Predasites, Helminth Charger).
    KubrowPets(KubrowPet),
    /// Crafting components (mutagens, antigens, cores, gyros).
    Pistols(PetComponent),
    /// Warframe exalted companions (Venari, Venari Prime).
    SpecialItems(WarframeCompanion),
}

/// Combat companion pet — has character stats and wikia data.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KubrowPet {
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

    #[serde(default)]
    pub polarities: Vec<Polarity>,

    // Wikia (guaranteed for combat pets)
    pub wiki_available: bool,
    pub wikia_url: String,
    pub wikia_thumbnail: String,
    pub introduced: Introduced,
    pub release_date: String,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,

    // Grouped props
    #[serde(flatten)]
    pub stats: CharacterStats,
}

/// Pet crafting component — has build fields and weapon-like stats.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PetComponent {
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

    // Build fields (guaranteed for components)
    pub build_price: i64,
    pub build_quantity: i64,
    pub build_time: i64,
    #[serde(default)]
    pub components: Vec<Component>,
    pub consume_on_build: bool,
    pub skip_build_time_price: i64,

    // Weapon-like stats (guaranteed for components)
    pub critical_chance: i64,
    pub critical_multiplier: i64,
    #[serde(default)]
    pub damage_per_shot: Vec<i64>,
    pub fire_rate: i64,
    pub omega_attenuation: i64,
    pub proc_chance: i64,
    pub total_damage: i64,

    pub exclude_from_codex: Option<bool>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

/// Warframe exalted companion (Venari, Venari Prime) — has character stats, always excluded from codex.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarframeCompanion {
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

    #[serde(default)]
    pub polarities: Vec<Polarity>,

    // Wikia (guaranteed)
    pub wiki_available: bool,
    pub wikia_url: String,
    pub wikia_thumbnail: String,
    pub introduced: Introduced,
    pub release_date: String,

    pub exclude_from_codex: bool,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,

    // Grouped props
    #[serde(flatten)]
    pub stats: CharacterStats,
}

// ── Trait implementations via match delegation ──

impl ProductCategory for PetEntry {
    fn get_product_categories(&self) -> Vec<String> {
        match self {
            PetEntry::KubrowPets(_) => vec!["KubrowPets".to_string()],
            PetEntry::Pistols(_) => vec!["Pistols".to_string()],
            PetEntry::SpecialItems(_) => vec!["SpecialItems".to_string()],
        }
    }
}

impl Item for PetEntry {
    fn unique_name(&self) -> &str {
        match self {
            PetEntry::KubrowPets(p) => &p.unique_name,
            PetEntry::Pistols(p) => &p.unique_name,
            PetEntry::SpecialItems(p) => &p.unique_name,
        }
    }
    fn name(&self) -> &str {
        match self {
            PetEntry::KubrowPets(p) => &p.name,
            PetEntry::Pistols(p) => &p.name,
            PetEntry::SpecialItems(p) => &p.name,
        }
    }
    fn category(&self) -> &str {
        match self {
            PetEntry::KubrowPets(p) => &p.category,
            PetEntry::Pistols(p) => &p.category,
            PetEntry::SpecialItems(p) => &p.category,
        }
    }
    fn type_field(&self) -> &str {
        match self {
            PetEntry::KubrowPets(p) => &p.type_field,
            PetEntry::Pistols(p) => &p.type_field,
            PetEntry::SpecialItems(p) => &p.type_field,
        }
    }
    fn image_name(&self) -> Option<&str> {
        match self {
            PetEntry::KubrowPets(p) => Some(&p.image_name),
            PetEntry::Pistols(p) => Some(&p.image_name),
            PetEntry::SpecialItems(p) => Some(&p.image_name),
        }
    }
    fn tradable(&self) -> bool {
        match self {
            PetEntry::KubrowPets(p) => p.tradable,
            PetEntry::Pistols(p) => p.tradable,
            PetEntry::SpecialItems(p) => p.tradable,
        }
    }
    fn masterable(&self) -> bool {
        match self {
            PetEntry::KubrowPets(p) => p.masterable,
            PetEntry::Pistols(p) => p.masterable,
            PetEntry::SpecialItems(p) => p.masterable,
        }
    }
    fn patchlogs(&self) -> &[Patchlog] {
        match self {
            PetEntry::KubrowPets(p) => &p.patchlogs,
            PetEntry::Pistols(p) => &p.patchlogs,
            PetEntry::SpecialItems(p) => &p.patchlogs,
        }
    }
}

impl Droppable for PetEntry {
    fn drops(&self) -> &[Drop] {
        match self {
            PetEntry::KubrowPets(p) => &p.drops,
            PetEntry::SpecialItems(p) => &p.drops,
            PetEntry::Pistols(_) => &[],
        }
    }
}

impl Buildable for PetEntry {
    fn build_price(&self) -> Option<i64> {
        match self {
            PetEntry::Pistols(p) => Some(p.build_price),
            _ => None,
        }
    }
    fn build_quantity(&self) -> Option<i64> {
        match self {
            PetEntry::Pistols(p) => Some(p.build_quantity),
            _ => None,
        }
    }
    fn build_time(&self) -> Option<i64> {
        match self {
            PetEntry::Pistols(p) => Some(p.build_time),
            _ => None,
        }
    }
    fn skip_build_time_price(&self) -> Option<i64> {
        match self {
            PetEntry::Pistols(p) => Some(p.skip_build_time_price),
            _ => None,
        }
    }
    fn consume_on_build(&self) -> Option<bool> {
        match self {
            PetEntry::Pistols(p) => Some(p.consume_on_build),
            _ => None,
        }
    }
    fn mastery_req(&self) -> Option<i64> {
        match self {
            PetEntry::KubrowPets(p) => Some(p.mastery_req),
            PetEntry::Pistols(p) => Some(p.mastery_req),
            PetEntry::SpecialItems(p) => Some(p.mastery_req),
        }
    }
    fn market_cost(&self) -> Option<i64> {
        None
    }
    fn bp_cost(&self) -> Option<i64> {
        None
    }
    fn components(&self) -> &[Component] {
        match self {
            PetEntry::Pistols(p) => &p.components,
            _ => &[],
        }
    }
}

impl WikiaLinked for PetEntry {
    fn wiki_available(&self) -> Option<bool> {
        match self {
            PetEntry::KubrowPets(p) => Some(p.wiki_available),
            PetEntry::SpecialItems(p) => Some(p.wiki_available),
            PetEntry::Pistols(_) => None,
        }
    }
    fn wikia_url(&self) -> Option<&str> {
        match self {
            PetEntry::KubrowPets(p) => Some(&p.wikia_url),
            PetEntry::SpecialItems(p) => Some(&p.wikia_url),
            PetEntry::Pistols(_) => None,
        }
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        match self {
            PetEntry::KubrowPets(p) => Some(&p.wikia_thumbnail),
            PetEntry::SpecialItems(p) => Some(&p.wikia_thumbnail),
            PetEntry::Pistols(_) => None,
        }
    }
    fn introduced(&self) -> Option<&Introduced> {
        match self {
            PetEntry::KubrowPets(p) => Some(&p.introduced),
            PetEntry::SpecialItems(p) => Some(&p.introduced),
            PetEntry::Pistols(_) => None,
        }
    }
    fn release_date(&self) -> Option<&str> {
        match self {
            PetEntry::KubrowPets(p) => Some(&p.release_date),
            PetEntry::SpecialItems(p) => Some(&p.release_date),
            PetEntry::Pistols(_) => None,
        }
    }
}

impl Equippable for PetEntry {
    fn polarities(&self) -> &[Polarity] {
        match self {
            PetEntry::KubrowPets(p) => &p.polarities,
            PetEntry::SpecialItems(p) => &p.polarities,
            PetEntry::Pistols(_) => &[],
        }
    }
    fn slot(&self) -> Option<&crate::itemdata::enums::Slot> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_pet() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/pet_test.json"
        ));

        let rec: PetEntry = from_str(json_data).unwrap();

        match &rec {
            PetEntry::KubrowPets(p) => {
                assert_eq!(
                    p.unique_name,
                    "/Lotus/Types/Game/CatbrowPet/MirrorCatbrowPetPowerSuit"
                );
                assert_eq!(p.stats.health, 310);
                assert_eq!(p.stats.shield, 270);
                assert_eq!(p.stats.armor, 300);
            }
            _ => panic!("Expected KubrowPets variant"),
        }
    }
}

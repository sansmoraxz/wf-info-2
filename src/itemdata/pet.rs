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
use crate::itemdata::traits::{Buildable, Droppable, Equippable, Item, WikiaLinked};

pub type Root = Vec<Pet>;

/// Pet entry, discriminated by `productCategory`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "productCategory")]
pub enum Pet {
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

    // Character stats (guaranteed for combat pets)
    pub health: i64,
    pub shield: i64,
    pub armor: i64,
    pub power: i64,
    pub stamina: i64,

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

    // Character stats (guaranteed)
    pub health: i64,
    pub shield: i64,
    pub armor: i64,
    pub power: i64,
    pub stamina: i64,

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
}

// ── Trait implementations via match delegation ──

impl ProductCategory for Pet {
    fn get_product_categories(&self) -> Vec<String> {
        match self {
            Pet::KubrowPets(_) => vec!["KubrowPets".to_string()],
            Pet::Pistols(_) => vec!["Pistols".to_string()],
            Pet::SpecialItems(_) => vec!["SpecialItems".to_string()],
        }
    }
}

impl Item for Pet {
    fn unique_name(&self) -> &str {
        match self {
            Pet::KubrowPets(p) => &p.unique_name,
            Pet::Pistols(p) => &p.unique_name,
            Pet::SpecialItems(p) => &p.unique_name,
        }
    }
    fn name(&self) -> &str {
        match self {
            Pet::KubrowPets(p) => &p.name,
            Pet::Pistols(p) => &p.name,
            Pet::SpecialItems(p) => &p.name,
        }
    }
    fn category(&self) -> &str {
        match self {
            Pet::KubrowPets(p) => &p.category,
            Pet::Pistols(p) => &p.category,
            Pet::SpecialItems(p) => &p.category,
        }
    }
    fn type_field(&self) -> &str {
        match self {
            Pet::KubrowPets(p) => &p.type_field,
            Pet::Pistols(p) => &p.type_field,
            Pet::SpecialItems(p) => &p.type_field,
        }
    }
    fn image_name(&self) -> Option<&str> {
        match self {
            Pet::KubrowPets(p) => Some(&p.image_name),
            Pet::Pistols(p) => Some(&p.image_name),
            Pet::SpecialItems(p) => Some(&p.image_name),
        }
    }
    fn tradable(&self) -> bool {
        match self {
            Pet::KubrowPets(p) => p.tradable,
            Pet::Pistols(p) => p.tradable,
            Pet::SpecialItems(p) => p.tradable,
        }
    }
    fn masterable(&self) -> bool {
        match self {
            Pet::KubrowPets(p) => p.masterable,
            Pet::Pistols(p) => p.masterable,
            Pet::SpecialItems(p) => p.masterable,
        }
    }
    fn patchlogs(&self) -> &[Patchlog] {
        match self {
            Pet::KubrowPets(p) => &p.patchlogs,
            Pet::Pistols(p) => &p.patchlogs,
            Pet::SpecialItems(p) => &p.patchlogs,
        }
    }
}

impl Droppable for Pet {
    fn drops(&self) -> &[Drop] {
        match self {
            Pet::KubrowPets(p) => &p.drops,
            Pet::SpecialItems(p) => &p.drops,
            Pet::Pistols(_) => &[],
        }
    }
}

impl Buildable for Pet {
    fn build_price(&self) -> Option<i64> {
        match self {
            Pet::Pistols(p) => Some(p.build_price),
            _ => None,
        }
    }
    fn build_quantity(&self) -> Option<i64> {
        match self {
            Pet::Pistols(p) => Some(p.build_quantity),
            _ => None,
        }
    }
    fn build_time(&self) -> Option<i64> {
        match self {
            Pet::Pistols(p) => Some(p.build_time),
            _ => None,
        }
    }
    fn skip_build_time_price(&self) -> Option<i64> {
        match self {
            Pet::Pistols(p) => Some(p.skip_build_time_price),
            _ => None,
        }
    }
    fn consume_on_build(&self) -> Option<bool> {
        match self {
            Pet::Pistols(p) => Some(p.consume_on_build),
            _ => None,
        }
    }
    fn mastery_req(&self) -> Option<i64> {
        match self {
            Pet::KubrowPets(p) => Some(p.mastery_req),
            Pet::Pistols(p) => Some(p.mastery_req),
            Pet::SpecialItems(p) => Some(p.mastery_req),
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
            Pet::Pistols(p) => &p.components,
            _ => &[],
        }
    }
}

impl WikiaLinked for Pet {
    fn wiki_available(&self) -> Option<bool> {
        match self {
            Pet::KubrowPets(p) => Some(p.wiki_available),
            Pet::SpecialItems(p) => Some(p.wiki_available),
            Pet::Pistols(_) => None,
        }
    }
    fn wikia_url(&self) -> Option<&str> {
        match self {
            Pet::KubrowPets(p) => Some(&p.wikia_url),
            Pet::SpecialItems(p) => Some(&p.wikia_url),
            Pet::Pistols(_) => None,
        }
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        match self {
            Pet::KubrowPets(p) => Some(&p.wikia_thumbnail),
            Pet::SpecialItems(p) => Some(&p.wikia_thumbnail),
            Pet::Pistols(_) => None,
        }
    }
    fn introduced(&self) -> Option<&Introduced> {
        match self {
            Pet::KubrowPets(p) => Some(&p.introduced),
            Pet::SpecialItems(p) => Some(&p.introduced),
            Pet::Pistols(_) => None,
        }
    }
    fn release_date(&self) -> Option<&str> {
        match self {
            Pet::KubrowPets(p) => Some(&p.release_date),
            Pet::SpecialItems(p) => Some(&p.release_date),
            Pet::Pistols(_) => None,
        }
    }
}

impl Equippable for Pet {
    fn polarities(&self) -> &[Polarity] {
        match self {
            Pet::KubrowPets(p) => &p.polarities,
            Pet::SpecialItems(p) => &p.polarities,
            Pet::Pistols(_) => &[],
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

        let rec: Pet = from_str(json_data).unwrap();

        match &rec {
            Pet::KubrowPets(p) => {
                assert_eq!(
                    p.unique_name,
                    "/Lotus/Types/Game/CatbrowPet/MirrorCatbrowPetPowerSuit"
                );
                assert_eq!(p.health, 310);
                assert_eq!(p.shield, 270);
                assert_eq!(p.armor, 300);
            }
            _ => panic!("Expected KubrowPets variant"),
        }
    }
}

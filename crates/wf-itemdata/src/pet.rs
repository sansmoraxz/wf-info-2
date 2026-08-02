//! Companion pet item data.
//!
//! Pets are discriminated by `productCategory`:
//! - KubrowPets: Combat companions (Kavats, Kubrows, Vulpaphylas, etc.)
//! - Pistols: Crafting components (mutagens, antigens, cores, gyros)
//! - SpecialItems: Warframe companions (Venari, Venari Prime)

use serde::{Deserialize, Serialize};

use crate::ProductCategory;
use crate::common::{Drop, Introduced, Patchlog};
use crate::components::Component;
use crate::enums::{PetType, Polarity, Slot};
use crate::props::{
    BuildableProps, CharacterStats, ItemDetailProps, ItemIdentityProps, TradableProps,
};
use crate::traits::{Buildable, Droppable, Equippable, Item, WikiaLinked};

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
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: PetType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,
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
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: PetType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,
    pub mastery_req: i64,

    #[serde(flatten)]
    pub build: BuildableProps,

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
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: PetType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,
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
            Self::KubrowPets(_) => vec!["KubrowPets".to_owned()],
            Self::Pistols(_) => vec!["MiscItems".to_owned()],
            Self::SpecialItems(_) => vec!["SpecialItems".to_owned()],
        }
    }
}

impl Item for PetEntry {
    fn unique_name(&self) -> &str {
        match self {
            Self::KubrowPets(p) => &p.identity.unique_name,
            Self::Pistols(p) => &p.identity.unique_name,
            Self::SpecialItems(p) => &p.identity.unique_name,
        }
    }
    fn name(&self) -> &str {
        match self {
            Self::KubrowPets(p) => &p.identity.name,
            Self::Pistols(p) => &p.identity.name,
            Self::SpecialItems(p) => &p.identity.name,
        }
    }
    fn category(&self) -> &str {
        match self {
            Self::KubrowPets(p) => &p.identity.category,
            Self::Pistols(p) => &p.identity.category,
            Self::SpecialItems(p) => &p.identity.category,
        }
    }
    fn type_field(&self) -> &str {
        match self {
            Self::KubrowPets(p) => p.type_field.as_ref(),
            Self::Pistols(p) => p.type_field.as_ref(),
            Self::SpecialItems(p) => p.type_field.as_ref(),
        }
    }
    fn image_name(&self) -> Option<&str> {
        match self {
            Self::KubrowPets(p) => p.detail.image_name.as_deref(),
            Self::Pistols(p) => p.detail.image_name.as_deref(),
            Self::SpecialItems(p) => p.detail.image_name.as_deref(),
        }
    }
    fn tradable(&self) -> bool {
        match self {
            Self::KubrowPets(p) => p.trade.tradable,
            Self::Pistols(p) => p.trade.tradable,
            Self::SpecialItems(p) => p.trade.tradable,
        }
    }
    fn masterable(&self) -> bool {
        match self {
            Self::KubrowPets(p) => p.trade.masterable,
            Self::Pistols(p) => p.trade.masterable,
            Self::SpecialItems(p) => p.trade.masterable,
        }
    }
    fn patchlogs(&self) -> &[Patchlog] {
        match self {
            Self::KubrowPets(p) => &p.patchlogs,
            Self::Pistols(p) => &p.patchlogs,
            Self::SpecialItems(p) => &p.patchlogs,
        }
    }
}

impl Droppable for PetEntry {
    fn drops(&self) -> &[Drop] {
        match self {
            Self::KubrowPets(p) => &p.drops,
            Self::SpecialItems(p) => &p.drops,
            Self::Pistols(_) => &[],
        }
    }
}

impl Buildable for PetEntry {
    fn build_price(&self) -> Option<i64> {
        match self {
            Self::Pistols(p) => p.build.build_price,
            _ => None,
        }
    }
    fn build_quantity(&self) -> Option<i64> {
        match self {
            Self::Pistols(p) => p.build.build_quantity,
            _ => None,
        }
    }
    fn build_time(&self) -> Option<i64> {
        match self {
            Self::Pistols(p) => p.build.build_time,
            _ => None,
        }
    }
    fn skip_build_time_price(&self) -> Option<i64> {
        match self {
            Self::Pistols(p) => p.build.skip_build_time_price,
            _ => None,
        }
    }
    fn consume_on_build(&self) -> Option<bool> {
        match self {
            Self::Pistols(p) => p.build.consume_on_build,
            _ => None,
        }
    }
    fn mastery_req(&self) -> Option<i64> {
        match self {
            Self::KubrowPets(p) => Some(p.mastery_req),
            Self::Pistols(p) => Some(p.mastery_req),
            Self::SpecialItems(p) => Some(p.mastery_req),
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
            Self::Pistols(p) => &p.build.components,
            _ => &[],
        }
    }
}

impl WikiaLinked for PetEntry {
    fn wiki_available(&self) -> Option<bool> {
        match self {
            Self::KubrowPets(p) => Some(p.wiki_available),
            Self::SpecialItems(p) => Some(p.wiki_available),
            Self::Pistols(_) => None,
        }
    }
    fn wikia_url(&self) -> Option<&str> {
        match self {
            Self::KubrowPets(p) => Some(&p.wikia_url),
            Self::SpecialItems(p) => Some(&p.wikia_url),
            Self::Pistols(_) => None,
        }
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        match self {
            Self::KubrowPets(p) => Some(&p.wikia_thumbnail),
            Self::SpecialItems(p) => Some(&p.wikia_thumbnail),
            Self::Pistols(_) => None,
        }
    }
    fn introduced(&self) -> Option<&Introduced> {
        match self {
            Self::KubrowPets(p) => Some(&p.introduced),
            Self::SpecialItems(p) => Some(&p.introduced),
            Self::Pistols(_) => None,
        }
    }
    fn release_date(&self) -> Option<&str> {
        match self {
            Self::KubrowPets(p) => Some(&p.release_date),
            Self::SpecialItems(p) => Some(&p.release_date),
            Self::Pistols(_) => None,
        }
    }
}

impl Equippable for PetEntry {
    fn polarities(&self) -> &[Polarity] {
        match self {
            Self::KubrowPets(p) => &p.polarities,
            Self::SpecialItems(p) => &p.polarities,
            Self::Pistols(_) => &[],
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
    fn test_deserialize_pet() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/pet_test.json"
        ));

        let rec: PetEntry = from_str(json_data).unwrap();

        match &rec {
            PetEntry::KubrowPets(p) => {
                assert_eq!(
                    p.identity.unique_name,
                    "/Lotus/Types/Game/CatbrowPet/MirrorCatbrowPetPowerSuit"
                );
                assert_eq!(p.identity.name, "Adarza Kavat");
                assert_eq!(p.identity.category, "Pets");
                assert!(!p.trade.tradable);
                assert!(p.trade.masterable);

                // Character stats
                assert_eq!(p.stats.health, 310);
                assert_eq!(p.stats.shield, 270);
                assert_eq!(p.stats.armor, 300);
                assert_eq!(p.stats.power, 100);

                // Has wikia link
                assert!(p.wiki_available);
                assert!(!p.wikia_url.is_empty());
            }
            _ => panic!("Expected KubrowPets variant"),
        }
    }

    #[test]
    fn test_deserialize_pet_component() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/pet_test_2.json"
        ));

        let rec: PetEntry = from_str(json_data).unwrap();

        match &rec {
            PetEntry::Pistols(p) => {
                assert_eq!(p.identity.name, "Adlet Core");
                assert_eq!(p.identity.category, "Pets");
                assert!(!p.trade.tradable);
                assert_eq!(p.build.build_price, Some(50000));
                assert_eq!(p.build.build_quantity, Some(1));
                assert_eq!(p.build.components.len(), 5);
                assert_eq!(p.total_damage, 0);
            }
            _ => panic!("Expected Pistols variant"),
        }
    }

    #[test]
    fn test_deserialize_pet_warframe_companion() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/pet_test_3.json"
        ));

        let rec: PetEntry = from_str(json_data).unwrap();

        match &rec {
            PetEntry::SpecialItems(p) => {
                assert_eq!(p.identity.name, "Venari");
                assert_eq!(p.identity.category, "Pets");
                assert!(!p.trade.tradable);
                assert!(p.exclude_from_codex);
                assert_eq!(p.stats.health, 900);
                assert_eq!(p.stats.armor, 350);
                assert_eq!(p.stats.shield, 0);
                assert_eq!(p.stats.power, 100);
            }
            _ => panic!("Expected SpecialItems variant"),
        }
    }
}

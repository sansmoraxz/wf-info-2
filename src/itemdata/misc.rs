//! Miscellaneous item data (catch-all category).

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::itemdata::common::{Drop, Introduced, Patchlog};
use crate::itemdata::components::Component;
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::traits::{Buildable, Droppable, Item, WikiaLinked};
use crate::itemdata::ProductCategory;

pub type Root = Vec<Misc>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Misc {
    // Core identity
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image_name: Option<String>,
    pub description: Option<String>,

    // Tradable
    pub tradable: bool,
    pub masterable: bool,

    // Misc-specific
    pub show_in_inventory: Option<bool>,
    pub required: Option<i64>,
    pub standing: Option<i64>,
    pub item_count: Option<i64>,
    pub probability: Option<f64>,
    pub rarity: Option<String>,
    pub reward_name: Option<String>,
    pub tier: Option<i64>,
    pub fusion_points: Option<i64>,
    #[serde(default)]
    pub parents: Vec<String>,
    pub exclude_from_codex: Option<bool>,

    // Weapon stats (for weapon-like misc items)
    pub damage: Option<DamageBreakdown>,
    #[serde(default)]
    pub damage_per_shot: Vec<f64>,
    pub total_damage: Option<f64>,
    pub critical_chance: Option<f64>,
    pub critical_multiplier: Option<f64>,
    pub proc_chance: Option<f64>,
    pub fire_rate: Option<f64>,
    #[serde(default)]
    pub attacks: Vec<Attack>,

    // Gun-specific
    pub accuracy: Option<f64>,
    pub magazine_size: Option<i64>,
    pub reload_time: Option<f64>,
    pub multishot: Option<i64>,
    pub trigger: Option<String>,
    pub noise: Option<String>,

    // Melee-specific
    pub blocking_angle: Option<i64>,
    pub combo_duration: Option<i64>,
    pub follow_through: Option<f64>,
    pub range: Option<f64>,
    pub slam_attack: Option<i64>,
    pub slam_radial_damage: Option<i64>,
    pub slam_radius: Option<i64>,
    pub slide_attack: Option<i64>,
    pub heavy_attack_damage: Option<i64>,
    pub heavy_slam_attack: Option<i64>,
    pub heavy_slam_radial_damage: Option<i64>,
    pub heavy_slam_radius: Option<i64>,
    pub stance_polarity: Option<String>,
    pub wind_up: Option<f64>,

    // Disposition
    pub disposition: Option<i64>,
    pub omega_attenuation: Option<f64>,
    pub prime_omega_attenuation: Option<f64>,

    // Equippable
    pub slot: Option<i64>,
    #[serde(default)]
    pub polarities: Vec<String>,
    pub mastery_req: Option<i64>,

    // Buildable
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub skip_build_time_price: Option<i64>,
    pub consume_on_build: Option<bool>,
    #[serde(default)]
    pub components: Vec<Component>,

    // Prime/vault
    pub vaulted: Option<bool>,

    // Wikia
    pub wiki_available: Option<bool>,
    pub wikia_url: Option<String>,
    pub wikia_thumbnail: Option<String>,
    pub introduced: Option<Introduced>,
    pub release_date: Option<String>,
    pub product_category: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,

    // Railjack-specific
    pub bin_capacity: Option<i64>,
    pub bin_count: Option<i64>,
    pub capacity_multiplier: Option<Vec<i64>>,
    pub durability: Option<i64>,
    pub fill_rate: Option<f64>,
    pub repair_rate: Option<i64>,
    #[serde(default)]
    pub specialities: Vec<Value>, // NOTE: observed to be empty array or null

    // Droppable
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Misc {
    fn get_product_categories(&self) -> Vec<String> {
        let s = match &self.product_category {
            Some(v) => v.as_str(),
            None => "MiscItems",
        };
        vec![s.into()]
    }
}

impl Item for Misc {
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
        self.image_name.as_deref()
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

impl Droppable for Misc {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl Buildable for Misc {
    fn build_price(&self) -> Option<i64> {
        self.build_price
    }
    fn build_quantity(&self) -> Option<i64> {
        self.build_quantity
    }
    fn build_time(&self) -> Option<i64> {
        self.build_time
    }
    fn skip_build_time_price(&self) -> Option<i64> {
        self.skip_build_time_price
    }
    fn consume_on_build(&self) -> Option<bool> {
        self.consume_on_build
    }
    fn mastery_req(&self) -> Option<i64> {
        self.mastery_req
    }
    fn market_cost(&self) -> Option<i64> {
        None
    }
    fn bp_cost(&self) -> Option<i64> {
        None
    }
    fn components(&self) -> &[Component] {
        &self.components
    }
}

impl WikiaLinked for Misc {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_misc() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/misc_test.json"
        ));

        let rec: Misc = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Gameplay/NarmerSorties/ArchonCrystalBorealMythic"
        );
    }
}

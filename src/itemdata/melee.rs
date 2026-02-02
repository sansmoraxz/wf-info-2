//! Melee weapon item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::common::{Drop, Introduced, Patchlog};
use crate::itemdata::components::Component;
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::traits::{
    Buildable, Droppable, Equippable, Item, MeleeWeapon, Prime, Weapon, WikiaLinked,
};
use crate::itemdata::ProductCategory;

pub type Root = Vec<Melee>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Melee {
    // Core identity
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image_name: String,
    pub description: String,

    // Tradable
    pub tradable: bool,
    pub masterable: bool,

    // Weapon stats
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    pub damage: Option<DamageBreakdown>,
    #[serde(default)]
    pub damage_per_shot: Vec<f64>,
    pub disposition: Option<i64>,
    pub fire_rate: f64,
    pub omega_attenuation: f64,
    pub proc_chance: f64,
    pub total_damage: f64,
    #[serde(default)]
    pub attacks: Vec<Attack>,

    // Melee-specific
    pub blocking_angle: Option<i64>,
    pub combo_duration: Option<i64>,
    pub follow_through: Option<f64>,
    pub range: Option<f64>,
    pub stance_polarity: Option<String>,
    pub slam_attack: Option<i64>,
    pub slam_radial_damage: Option<i64>,
    pub slam_radius: Option<i64>,
    pub slide_attack: Option<i64>,
    pub heavy_attack_damage: Option<i64>,
    pub heavy_slam_attack: Option<i64>,
    pub heavy_slam_radial_damage: Option<i64>,
    pub heavy_slam_radius: Option<i64>,
    pub wind_up: Option<f64>,

    // Buildable
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub skip_build_time_price: Option<i64>,
    pub consume_on_build: Option<bool>,
    pub mastery_req: i64,
    pub market_cost: Option<i64>,
    pub bp_cost: Option<i64>,
    #[serde(default)]
    pub components: Vec<Component>,

    // Equippable
    #[serde(default)]
    pub polarities: Vec<String>,
    pub slot: i64,
    #[serde(default)]
    pub tags: Vec<String>,

    // Prime/vault
    #[serde(default)]
    pub is_prime: bool,
    pub vaulted: Option<bool>,
    pub vault_date: Option<String>,
    pub estimated_vault_date: Option<String>,

    // Wikia
    pub wiki_available: Option<bool>,
    pub wikia_url: Option<String>,
    pub wikia_thumbnail: Option<String>,
    pub introduced: Option<Introduced>,
    pub release_date: Option<String>,
    pub product_category: String,
    pub max_level_cap: Option<i64>,
    pub exclude_from_codex: Option<bool>,

    // Misc
    pub item_count: Option<i64>,
    #[serde(default)]
    pub parents: Vec<String>,

    // Droppable
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Melee {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.clone()]
    }
}

impl Item for Melee {
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

impl Droppable for Melee {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl Buildable for Melee {
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
        Some(self.mastery_req)
    }
    fn market_cost(&self) -> Option<i64> {
        self.market_cost
    }
    fn bp_cost(&self) -> Option<i64> {
        self.bp_cost
    }
    fn components(&self) -> &[Component] {
        &self.components
    }
}

impl Prime for Melee {
    fn is_prime(&self) -> bool {
        self.is_prime
    }
    fn vaulted(&self) -> Option<bool> {
        self.vaulted
    }
    fn vault_date(&self) -> Option<&str> {
        self.vault_date.as_deref()
    }
    fn estimated_vault_date(&self) -> Option<&str> {
        self.estimated_vault_date.as_deref()
    }
}

impl WikiaLinked for Melee {
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

impl Weapon for Melee {
    fn critical_chance(&self) -> f64 {
        self.critical_chance
    }
    fn critical_multiplier(&self) -> f64 {
        self.critical_multiplier
    }
    fn damage(&self) -> Option<&DamageBreakdown> {
        self.damage.as_ref()
    }
    fn damage_per_shot(&self) -> &[f64] {
        &self.damage_per_shot
    }
    fn total_damage(&self) -> f64 {
        self.total_damage
    }
    fn proc_chance(&self) -> f64 {
        self.proc_chance
    }
    fn fire_rate(&self) -> f64 {
        self.fire_rate
    }
    fn disposition(&self) -> Option<i64> {
        self.disposition
    }
    fn omega_attenuation(&self) -> f64 {
        self.omega_attenuation
    }
    fn attacks(&self) -> &[Attack] {
        &self.attacks
    }
}

impl MeleeWeapon for Melee {
    fn blocking_angle(&self) -> Option<i64> {
        self.blocking_angle
    }
    fn combo_duration(&self) -> Option<i64> {
        self.combo_duration
    }
    fn follow_through(&self) -> Option<f64> {
        self.follow_through
    }
    fn range(&self) -> Option<f64> {
        self.range
    }
    fn stance_polarity(&self) -> Option<&str> {
        self.stance_polarity.as_deref()
    }
    fn slam_attack(&self) -> Option<i64> {
        self.slam_attack
    }
    fn heavy_attack_damage(&self) -> Option<i64> {
        self.heavy_attack_damage
    }
}

impl Equippable for Melee {
    fn polarities(&self) -> &[String] {
        &self.polarities
    }
    fn slot(&self) -> Option<i64> {
        Some(self.slot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_archmelee() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/melee_test.json"
        ));

        let rec: Melee = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/Grineer/Melee/GrineerTylAxeAndBoar/RegorAxeShield"
        );
    }
}

//! Archwing melee weapon item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Introduced, Patchlog};
use crate::itemdata::components::Component;
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::enums::Polarity;
use crate::itemdata::traits::{
    Buildable, Equippable, Item, MeleeWeapon, Prime, Weapon, WikiaLinked,
};

pub type Root = Vec<ArchMelee>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchMelee {
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
    pub damage: DamageBreakdown,
    #[serde(default)]
    pub damage_per_shot: Vec<f64>,
    pub total_damage: f64,
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    pub proc_chance: f64,
    pub fire_rate: f64,
    #[serde(default)]
    pub attacks: Vec<Attack>,

    // Melee-specific
    pub blocking_angle: i64,
    pub combo_duration: i64,
    pub follow_through: f64,
    pub range: f64,
    pub slam_attack: i64,
    pub slam_radial_damage: i64,
    pub slam_radius: i64,
    pub slide_attack: i64,
    pub heavy_attack_damage: i64,
    pub heavy_slam_attack: i64,

    // Disposition
    pub disposition: i64,
    pub omega_attenuation: f64,

    // Equippable
    pub slot: i64,
    #[serde(default)]
    pub polarities: Vec<Polarity>,
    pub mastery_req: i64,

    // Buildable
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

    // Wikia
    pub wiki_available: bool,
    pub wikia_url: String,
    pub wikia_thumbnail: String,
    pub introduced: Introduced,
    pub release_date: String,
    pub product_category: String,
    #[serde(default)]
    pub tags: Vec<String>,

    // Droppable
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for ArchMelee {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.clone()]
    }
}

impl Item for ArchMelee {
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

impl Buildable for ArchMelee {
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

impl Prime for ArchMelee {
    fn is_prime(&self) -> bool {
        self.is_prime
    }
    fn vaulted(&self) -> Option<bool> {
        None
    }
    fn vault_date(&self) -> Option<&str> {
        None
    }
    fn estimated_vault_date(&self) -> Option<&str> {
        None
    }
}

impl WikiaLinked for ArchMelee {
    fn wiki_available(&self) -> Option<bool> {
        Some(self.wiki_available)
    }
    fn wikia_url(&self) -> Option<&str> {
        Some(&self.wikia_url)
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        Some(&self.wikia_thumbnail)
    }
    fn introduced(&self) -> Option<&Introduced> {
        Some(&self.introduced)
    }
    fn release_date(&self) -> Option<&str> {
        Some(&self.release_date)
    }
}

impl Weapon for ArchMelee {
    fn critical_chance(&self) -> f64 {
        self.critical_chance
    }
    fn critical_multiplier(&self) -> f64 {
        self.critical_multiplier
    }
    fn damage(&self) -> Option<&DamageBreakdown> {
        Some(&self.damage)
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
        Some(self.disposition)
    }
    fn omega_attenuation(&self) -> f64 {
        self.omega_attenuation
    }
    fn attacks(&self) -> &[Attack] {
        &self.attacks
    }
}

impl MeleeWeapon for ArchMelee {
    fn blocking_angle(&self) -> Option<i64> {
        Some(self.blocking_angle)
    }
    fn combo_duration(&self) -> Option<i64> {
        Some(self.combo_duration)
    }
    fn follow_through(&self) -> Option<f64> {
        Some(self.follow_through)
    }
    fn range(&self) -> Option<f64> {
        Some(self.range)
    }
    fn stance_polarity(&self) -> Option<&str> {
        None
    }
    fn slam_attack(&self) -> Option<i64> {
        Some(self.slam_attack)
    }
    fn heavy_attack_damage(&self) -> Option<i64> {
        Some(self.heavy_attack_damage)
    }
}

impl Equippable for ArchMelee {
    fn polarities(&self) -> &[Polarity] {
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
            "/testdata/itemdata/arch_melee_test.json"
        ));

        let rec: ArchMelee = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/Tenno/Archwing/Melee/ArchScythe/ArchScythe"
        );
    }
}

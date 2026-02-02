//! Sentinel weapon item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::common::{Introduced, Patchlog};
use crate::itemdata::components::Component;
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::enums::{Noise, Polarity, Trigger};
use crate::itemdata::props::WeaponTypeStats;
use crate::itemdata::traits::{Buildable, Equippable, Item, Prime, Weapon, WikiaLinked};
use crate::itemdata::ProductCategory;

pub type Root = Vec<SentinelWeapon>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentinelWeapon {
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

    // Gun-specific (optional for melee sentinel weapons)
    pub accuracy: Option<f64>,
    pub magazine_size: Option<i64>,
    pub reload_time: Option<f64>,
    pub multishot: Option<i64>,
    #[serde(default)]
    pub trigger: Option<Trigger>,
    #[serde(default)]
    pub noise: Option<Noise>,

    // Melee-specific (optional)
    pub blocking_angle: Option<i64>,

    // Sentinel-specific
    pub sentinel: bool,

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
    pub components: Option<Vec<Component>>,

    // Prime/vault
    #[serde(default)]
    pub is_prime: bool,
    pub vaulted: Option<bool>,

    // Wikia
    pub wiki_available: bool,
    pub wikia_url: String,
    pub wikia_thumbnail: String,
    pub introduced: Introduced,
    pub release_date: String,
    pub product_category: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub exclude_from_codex: Option<bool>,

    // Droppable
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for SentinelWeapon {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.clone()]
    }
}

impl SentinelWeapon {
    /// Get the computed weapon type classification.
    ///
    /// Sentinel weapons can be either ranged (guns) or melee.
    /// Returns `WeaponTypeStats::Ranged` for gun-type sentinel weapons,
    /// `WeaponTypeStats::Melee` for melee-type sentinel weapons.
    pub fn weapon_type_stats(&self) -> WeaponTypeStats {
        WeaponTypeStats::detect(
            self.accuracy,
            self.magazine_size,
            self.reload_time,
            self.multishot,
            self.noise.clone(),
            self.trigger.clone(),
            None, // projectile
            None, // flight
            self.blocking_angle,
            None, // combo_duration
            None, // follow_through
            None, // range
            None, // stance_polarity
            None, // slam_attack
            None, // slam_radial_damage
            None, // slam_radius
            None, // slide_attack
            None, // heavy_attack_damage
            None, // heavy_slam_attack
            None, // heavy_slam_radial_damage
            None, // heavy_slam_radius
            None, // wind_up
        )
    }

    /// Check if this is a ranged sentinel weapon
    pub fn is_ranged(&self) -> bool {
        self.weapon_type_stats().is_ranged()
    }

    /// Check if this is a melee sentinel weapon
    pub fn is_melee(&self) -> bool {
        self.weapon_type_stats().is_melee()
    }
}

impl Item for SentinelWeapon {
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

impl Buildable for SentinelWeapon {
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
        None
    }
    fn bp_cost(&self) -> Option<i64> {
        None
    }
    fn components(&self) -> &[Component] {
        match &self.components {
            Some(c) => c,
            None => &[],
        }
    }
}

impl Prime for SentinelWeapon {
    fn is_prime(&self) -> bool {
        self.is_prime
    }
    fn vaulted(&self) -> Option<bool> {
        self.vaulted
    }
    fn vault_date(&self) -> Option<&str> {
        None
    }
    fn estimated_vault_date(&self) -> Option<&str> {
        None
    }
}

impl WikiaLinked for SentinelWeapon {
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

impl Weapon for SentinelWeapon {
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

impl Equippable for SentinelWeapon {
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
    fn test_deserialize_sentinel_weapon() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/sentinel_weapon_test.json"
        ));

        let rec: SentinelWeapon = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Friendly/Pets/ZanukaPets/ZanukaPetMeleeWeaponIP"
        );
    }
}

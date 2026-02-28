//! Archwing gun item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Introduced, Patchlog};
use crate::itemdata::components::Component;
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::enums::{ArchGunProductCategory, Noise, Polarity, Slot, Trigger};
use crate::itemdata::traits::{
    Buildable, Equippable, Item, Prime, RangedWeapon, Weapon, WikiaLinked,
};

pub type Root = Vec<ArchGun>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchGun {
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
    pub accuracy: f64,
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    pub damage: DamageBreakdown,
    #[serde(default)]
    pub damage_per_shot: Vec<f64>,
    pub disposition: Option<i64>,
    pub fire_rate: f64,
    pub multishot: i64,
    #[serde(default)]
    pub noise: Noise,
    pub omega_attenuation: f64,
    pub proc_chance: f64,
    pub total_damage: i64,
    #[serde(default)]
    pub trigger: Trigger,
    #[serde(default)]
    pub attacks: Vec<Attack>,

    // Gun-specific
    pub magazine_size: i64,
    pub reload_time: f64,

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
    pub polarities: Vec<Polarity>,
    pub slot: Slot,
    #[serde(default)]
    pub tags: Vec<String>,

    // Prime/vault
    #[serde(default)]
    pub is_prime: bool,
    pub vaulted: Option<bool>,
    pub estimated_vault_date: Option<String>,

    // Wikia
    pub wiki_available: Option<bool>,
    pub wikia_thumbnail: Option<String>,
    pub wikia_url: Option<String>,
    pub introduced: Option<Introduced>,
    pub release_date: Option<String>,
    pub product_category: ArchGunProductCategory,
    pub max_level_cap: Option<i64>,

    // Droppable
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for ArchGun {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.as_str().to_string()]
    }
}

impl Item for ArchGun {
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

impl Buildable for ArchGun {
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

impl Prime for ArchGun {
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
        self.estimated_vault_date.as_deref()
    }
}

impl WikiaLinked for ArchGun {
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

impl Weapon for ArchGun {
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
        self.total_damage as f64
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

impl RangedWeapon for ArchGun {
    fn accuracy(&self) -> f64 {
        self.accuracy
    }
    fn multishot(&self) -> i64 {
        self.multishot
    }
    fn noise(&self) -> &str {
        self.noise.as_str()
    }
    fn trigger(&self) -> &str {
        self.trigger.as_str()
    }
    fn magazine_size(&self) -> Option<i64> {
        Some(self.magazine_size)
    }
    fn reload_time(&self) -> f64 {
        self.reload_time
    }
}

impl Equippable for ArchGun {
    fn polarities(&self) -> &[Polarity] {
        &self.polarities
    }
    fn slot(&self) -> Option<&Slot> {
        Some(&self.slot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_archgun() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/arch_gun_test.json"
        ));

        let rec: ArchGun = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/Tenno/Archwing/Primary/NokkoArchGun/NokkoArchGun"
        );
    }
}

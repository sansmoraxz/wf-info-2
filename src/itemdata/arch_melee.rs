//! Archwing melee weapon item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::Patchlog;
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::enums::{ArchMeleeProductCategory, Polarity, Slot};
use crate::itemdata::props::{BuildableProps, EquippableProps, PrimeProps, WeaponProps, WikiaProps};
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
    #[serde(flatten)]
    pub weapon: WeaponProps,

    // Melee-specific (always present 8/8; non-Optional for ArchMelee)
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

    // Equippable
    #[serde(flatten)]
    pub equip: EquippableProps,

    pub product_category: ArchMeleeProductCategory,

    // Droppable
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,

    // Grouped props
    #[serde(flatten)]
    pub build: BuildableProps,
    #[serde(flatten)]
    pub prime: PrimeProps,
    #[serde(flatten)]
    pub wikia: WikiaProps,
}

impl ProductCategory for ArchMelee {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.as_str().to_string()]
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
        self.build.build_price
    }
    fn build_quantity(&self) -> Option<i64> {
        self.build.build_quantity
    }
    fn build_time(&self) -> Option<i64> {
        self.build.build_time
    }
    fn skip_build_time_price(&self) -> Option<i64> {
        self.build.skip_build_time_price
    }
    fn consume_on_build(&self) -> Option<bool> {
        self.build.consume_on_build
    }
    fn mastery_req(&self) -> Option<i64> {
        self.build.mastery_req
    }
    fn market_cost(&self) -> Option<i64> {
        self.build.market_cost
    }
    fn bp_cost(&self) -> Option<i64> {
        self.build.bp_cost
    }
    fn components(&self) -> &[crate::itemdata::components::Component] {
        &self.build.components
    }
}

impl Prime for ArchMelee {
    fn is_prime(&self) -> bool {
        self.prime.is_prime
    }
    fn vaulted(&self) -> Option<bool> {
        self.prime.vaulted
    }
    fn vault_date(&self) -> Option<&str> {
        self.prime.vault_date.as_deref()
    }
    fn estimated_vault_date(&self) -> Option<&str> {
        self.prime.estimated_vault_date.as_deref()
    }
}

impl WikiaLinked for ArchMelee {
    fn wiki_available(&self) -> Option<bool> {
        self.wikia.wiki_available
    }
    fn wikia_url(&self) -> Option<&str> {
        self.wikia.wikia_url.as_deref()
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        self.wikia.wikia_thumbnail.as_deref()
    }
    fn introduced(&self) -> Option<&crate::itemdata::common::Introduced> {
        self.wikia.introduced.as_ref()
    }
    fn release_date(&self) -> Option<&str> {
        self.wikia.release_date.as_deref()
    }
}

impl Weapon for ArchMelee {
    fn critical_chance(&self) -> f64 {
        self.weapon.critical_chance
    }
    fn critical_multiplier(&self) -> f64 {
        self.weapon.critical_multiplier
    }
    fn damage(&self) -> Option<&DamageBreakdown> {
        self.weapon.damage.as_ref()
    }
    fn damage_per_shot(&self) -> &[f64] {
        &self.weapon.damage_per_shot
    }
    fn total_damage(&self) -> f64 {
        self.weapon.total_damage
    }
    fn proc_chance(&self) -> f64 {
        self.weapon.proc_chance
    }
    fn fire_rate(&self) -> f64 {
        self.weapon.fire_rate
    }
    fn disposition(&self) -> Option<i64> {
        self.weapon.disposition
    }
    fn omega_attenuation(&self) -> f64 {
        self.weapon.omega_attenuation
    }
    fn attacks(&self) -> &[Attack] {
        &self.weapon.attacks
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
        &self.equip.polarities
    }
    fn slot(&self) -> Option<&Slot> {
        self.equip.slot.as_ref()
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

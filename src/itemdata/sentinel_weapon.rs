//! Sentinel weapon item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::Patchlog;
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::enums::{Noise, Polarity, SentinelWeaponProductCategory, SentinelWeaponType, Slot, Trigger};
use crate::itemdata::props::{
    BuildableProps, EquippableProps, ItemDetailProps, ItemIdentityProps, PrimeProps, TradableProps,
    WeaponProps, WeaponTypeStats, WikiaProps,
};
use crate::itemdata::traits::{Buildable, Equippable, Item, Prime, Weapon, WikiaLinked};

pub type Root = Vec<SentinelWeapon>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentinelWeapon {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: SentinelWeaponType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    // Weapon stats
    #[serde(flatten)]
    pub weapon: WeaponProps,

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

    // Equippable
    #[serde(flatten)]
    pub equip: EquippableProps,

    pub product_category: SentinelWeaponProductCategory,
    #[serde(default)]
    pub exclude_from_codex: Option<bool>,

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

impl ProductCategory for SentinelWeapon {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.as_str().to_string()]
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
        &self.identity.unique_name
    }
    fn name(&self) -> &str {
        &self.identity.name
    }
    fn category(&self) -> &str {
        &self.identity.category
    }
    fn type_field(&self) -> &str {
        self.type_field.as_str()
    }
    fn image_name(&self) -> Option<&str> {
        self.detail.image_name.as_deref()
    }
    fn tradable(&self) -> bool {
        self.trade.tradable
    }
    fn masterable(&self) -> bool {
        self.trade.masterable
    }
    fn patchlogs(&self) -> &[Patchlog] {
        &self.patchlogs
    }
}

impl Buildable for SentinelWeapon {
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

impl Prime for SentinelWeapon {
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

impl WikiaLinked for SentinelWeapon {
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

impl Weapon for SentinelWeapon {
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

impl Equippable for SentinelWeapon {
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
    fn test_deserialize_sentinel_weapon() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/sentinel_weapon_test.json"
        ));

        let rec: SentinelWeapon = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Friendly/Pets/ZanukaPets/ZanukaPetMeleeWeaponIP"
        );
        assert_eq!(rec.identity.name, "Batoten");
        assert!(!rec.trade.tradable);
        assert!(rec.trade.masterable);

        // Weapon stats
        assert!((rec.weapon.critical_chance - 0.1).abs() < 0.01);
        assert!((rec.weapon.total_damage - 300.0).abs() < 0.01);
        assert_eq!(rec.weapon.damage_per_shot.len(), 20);

        assert!(!rec.prime.is_prime);
    }
}

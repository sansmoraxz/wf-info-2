//! Archwing gun item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::Patchlog;
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::enums::{ArchGunProductCategory, ArchGunType, Polarity, Slot};
use crate::itemdata::props::{
    BuildableProps, EquippableProps, GunProps, ItemDetailProps, ItemIdentityProps, PrimeProps,
    TradableProps, WeaponProps, WikiaProps,
};
use crate::itemdata::traits::{
    Buildable, Equippable, Item, Prime, RangedWeapon, Weapon, WikiaLinked,
};

pub type Root = Vec<ArchGun>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchGun {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: ArchGunType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    // Weapon stats
    #[serde(flatten)]
    pub weapon: WeaponProps,

    // Gun-specific
    #[serde(flatten)]
    pub gun: GunProps,

    // Equippable
    #[serde(flatten)]
    pub equip: EquippableProps,

    pub product_category: ArchGunProductCategory,
    pub max_level_cap: Option<i64>,

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

impl ProductCategory for ArchGun {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.as_str().to_string()]
    }
}

impl Item for ArchGun {
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

impl Buildable for ArchGun {
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

impl Prime for ArchGun {
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

impl WikiaLinked for ArchGun {
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

impl Weapon for ArchGun {
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

impl RangedWeapon for ArchGun {
    fn accuracy(&self) -> f64 {
        self.gun.accuracy
    }
    fn multishot(&self) -> i64 {
        self.gun.multishot
    }
    fn noise(&self) -> &str {
        self.gun.noise.as_str()
    }
    fn trigger(&self) -> &str {
        self.gun.trigger.as_str()
    }
    fn magazine_size(&self) -> Option<i64> {
        self.gun.magazine_size
    }
    fn reload_time(&self) -> f64 {
        self.gun.reload_time
    }
}

impl Equippable for ArchGun {
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
    fn test_deserialize_archgun() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/arch_gun_test.json"
        ));

        let rec: ArchGun = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Weapons/Tenno/Archwing/Primary/NokkoArchGun/NokkoArchGun"
        );
        assert_eq!(rec.identity.name, "Arbucep");
        assert_eq!(rec.identity.category, "Arch-Gun");
        assert!(!rec.trade.tradable);
        assert!(rec.trade.masterable);

        // Weapon stats
        assert!((rec.weapon.critical_chance - 0.1).abs() < 0.01);
        assert!((rec.weapon.total_damage - 130.0).abs() < 0.01);
        assert_eq!(rec.weapon.damage_per_shot.len(), 20);

        // Gun stats
        assert_eq!(rec.gun.magazine_size, Some(6));

        // Buildable
        assert_eq!(rec.build.build_price, Some(25000));
        assert_eq!(rec.build.components.len(), 5);
    }
}

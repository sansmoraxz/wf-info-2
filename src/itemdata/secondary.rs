//! Secondary weapon item data (pistols, thrown weapons, etc.).

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, Patchlog};
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::enums::{Polarity, SecondaryProductCategory, SecondaryType, Slot};
use crate::itemdata::props::{
    BuildableProps, EquippableProps, GunProps, ItemDetailProps, ItemIdentityProps, PrimeProps,
    TradableProps, WeaponProps, WikiaProps,
};
use crate::itemdata::traits::{
    Buildable, Droppable, Equippable, Item, Prime, RangedWeapon, Weapon, WikiaLinked,
};

pub type Root = Vec<Secondary>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Secondary {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: SecondaryType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    // Grouped weapon/gun/equip props
    #[serde(flatten)]
    pub weapon: WeaponProps,
    #[serde(flatten)]
    pub gun: GunProps,
    #[serde(flatten)]
    pub equip: EquippableProps,

    pub product_category: SecondaryProductCategory,
    pub max_level_cap: Option<i64>,

    // Misc
    pub item_count: Option<i64>,
    #[serde(default)]
    pub parents: Vec<String>,

    #[serde(default)]
    pub drops: Vec<Drop>,
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

impl ProductCategory for Secondary {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.as_str().to_string()]
    }
}

impl Item for Secondary {
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

impl Droppable for Secondary {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl Buildable for Secondary {
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

impl Prime for Secondary {
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

impl WikiaLinked for Secondary {
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

impl Weapon for Secondary {
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

impl RangedWeapon for Secondary {
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

impl Equippable for Secondary {
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
    fn test_deserialize_secondary() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/secondary_test.json"
        ));

        let rec: Secondary = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Weapons/ClanTech/Bio/AcidDartPistol"
        );
        assert_eq!(rec.identity.name, "Acrid");
        assert_eq!(rec.identity.category, "Secondary");
        assert_eq!(rec.type_field, SecondaryType::Pistol);
        assert!(!rec.trade.tradable);
        assert!(rec.trade.masterable);

        // Weapon stats
        assert!((rec.weapon.critical_chance - 0.05).abs() < 0.01);
        assert!((rec.weapon.total_damage - 43.0).abs() < 0.01);
        assert_eq!(rec.weapon.damage_per_shot.len(), 20);

        // Gun stats
        assert_eq!(rec.gun.magazine_size, Some(15));
        assert!((rec.gun.accuracy - 100.0).abs() < 0.01);

        // Buildable
        assert_eq!(rec.build.build_price, Some(30000));
        assert_eq!(rec.build.components.len(), 5);

        // Equippable
        assert_eq!(rec.equip.slot, Some(Slot::Secondary));
        assert!(!rec.prime.is_prime);
    }

    #[test]
    fn test_deserialize_secondary_prime() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/secondary_test_2.json"
        ));
        let rec: Secondary = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Lex Prime");
        assert!((rec.weapon.total_damage - 180.0).abs() < 0.01);
        assert!((rec.weapon.critical_chance - 0.25).abs() < 0.01);
        assert_eq!(rec.gun.magazine_size, Some(8));
        assert!(rec.prime.is_prime);
        assert_eq!(rec.prime.vaulted, Some(false));
    }

    #[test]
    fn test_deserialize_secondary_base() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/secondary_test_3.json"
        ));
        let rec: Secondary = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Furis");
        assert!((rec.weapon.total_damage - 20.0).abs() < 0.01);
        assert_eq!(rec.gun.magazine_size, Some(35));
        assert!(!rec.prime.is_prime);
    }
}

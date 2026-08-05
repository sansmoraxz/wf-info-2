//! Primary weapon item data (rifles, shotguns, bows, etc.).

use serde::{Deserialize, Serialize};

use crate::ProductCategory;
use crate::common::{Drop, Introduced, Patchlog};
use crate::components::Component;
use crate::damage::{Attack, DamageBreakdown};
use crate::enums::{Polarity, PrimaryProductCategory, PrimaryType, Slot};
use crate::props::{
    BuildableProps, EquippableProps, GunProps, ItemDetailProps, ItemIdentityProps, PrimeProps,
    TradableProps, WeaponProps, WikiaProps,
};
use crate::traits::{
    Buildable, Droppable, Equippable, Item, Prime, RangedWeapon, Weapon, WikiaLinked,
};

pub type Root = Vec<Primary>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Primary {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: PrimaryType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    pub product_category: PrimaryProductCategory,
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
    pub weapon: WeaponProps,
    #[serde(flatten)]
    pub gun: GunProps,
    #[serde(flatten)]
    pub equip: EquippableProps,
    #[serde(flatten)]
    pub build: BuildableProps,
    #[serde(flatten)]
    pub prime: PrimeProps,
    #[serde(flatten)]
    pub wikia: WikiaProps,
}

impl ProductCategory for Primary {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.as_ref().to_owned()]
    }
}

impl Item for Primary {
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
        self.type_field.as_ref()
    }
    fn image_name(&self) -> Option<&str> {
        self.detail.image_name.as_deref()
    }
    fn description(&self) -> Option<&str> {
        self.detail.description.as_deref()
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

impl Droppable for Primary {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl Buildable for Primary {
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
    fn components(&self) -> &[Component] {
        &self.build.components
    }
}

impl Prime for Primary {
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

impl WikiaLinked for Primary {
    fn wiki_available(&self) -> Option<bool> {
        self.wikia.wiki_available
    }
    fn wikia_url(&self) -> Option<&str> {
        self.wikia.wikia_url.as_deref()
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        self.wikia.wikia_thumbnail.as_deref()
    }
    fn introduced(&self) -> Option<&Introduced> {
        self.wikia.introduced.as_ref()
    }
    fn release_date(&self) -> Option<&str> {
        self.wikia.release_date.as_deref()
    }
}

impl Weapon for Primary {
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

impl RangedWeapon for Primary {
    fn accuracy(&self) -> f64 {
        self.gun.accuracy
    }
    fn multishot(&self) -> i64 {
        self.gun.multishot
    }
    fn noise(&self) -> &str {
        self.gun.noise.as_ref()
    }
    fn trigger(&self) -> &str {
        self.gun.trigger.as_ref()
    }
    fn magazine_size(&self) -> Option<i64> {
        self.gun.magazine_size
    }
    fn reload_time(&self) -> f64 {
        self.gun.reload_time
    }
}

impl Equippable for Primary {
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
    fn test_deserialize_primary() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/primary_test.json"
        ));

        let rec: Primary = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Weapons/Tenno/LongGuns/SapientPrimary/SapientPrimaryWeapon"
        );
        assert_eq!(rec.identity.name, "Acceltra");
        assert_eq!(rec.identity.category, "Primary");
        assert_eq!(rec.type_field, PrimaryType::Rifle);
        assert!(!rec.trade.tradable);
        assert!(rec.trade.masterable);

        // Weapon stats
        assert!((rec.weapon.critical_chance - 0.32).abs() < 0.01_f64);
        assert!((rec.weapon.total_damage - 70.0).abs() < 0.01_f64);
        assert!((rec.weapon.fire_rate - 12.0).abs() < 0.1_f64);
        assert_eq!(rec.weapon.damage_per_shot.len(), 20);

        // Gun stats
        assert_eq!(rec.gun.magazine_size, Some(48));
        assert!((rec.gun.accuracy - 23.53).abs() < 0.01_f64);

        // Buildable
        assert_eq!(rec.build.build_price, Some(25000));
        assert_eq!(rec.build.components.len(), 5);

        // Not prime
        assert!(!rec.prime.is_prime);

        // Equippable
        assert_eq!(rec.equip.slot, Some(Slot::Primary));
    }

    #[test]
    fn test_deserialize_primary_prime() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/primary_test_2.json"
        ));
        let rec: Primary = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Soma Prime");
        assert!((rec.weapon.total_damage - 12.0).abs() < 0.01_f64);
        assert!((rec.weapon.critical_chance - 0.3).abs() < 0.01_f64);
        assert_eq!(rec.gun.magazine_size, Some(200));
        assert!(rec.prime.is_prime);
        assert_eq!(rec.prime.vaulted, Some(true));
    }

    #[test]
    fn test_deserialize_primary_base() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/primary_test_3.json"
        ));
        let rec: Primary = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Braton");
        assert!((rec.weapon.total_damage - 24.0).abs() < 0.01_f64);
        assert_eq!(rec.gun.magazine_size, Some(45));
        assert!(!rec.prime.is_prime);
    }
}

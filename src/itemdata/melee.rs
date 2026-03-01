//! Melee weapon item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, Patchlog};
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::enums::{MeleeProductCategory, MeleeType, Polarity, Slot};
use crate::itemdata::props::{
    BuildableProps, EquippableProps, ItemDetailProps, ItemIdentityProps, MeleeProps, PrimeProps,
    TradableProps, WeaponProps, WikiaProps,
};
use crate::itemdata::traits::{
    Buildable, Droppable, Equippable, Item, MeleeWeapon, Prime, Weapon, WikiaLinked,
};

pub type Root = Vec<Melee>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Melee {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: MeleeType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    // Weapon stats
    #[serde(flatten)]
    pub weapon: WeaponProps,

    // Equippable
    #[serde(flatten)]
    pub equip: EquippableProps,

    pub product_category: MeleeProductCategory,
    pub max_level_cap: Option<i64>,
    pub exclude_from_codex: Option<bool>,

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
    #[serde(flatten)]
    pub melee: MeleeProps,
}

impl ProductCategory for Melee {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.as_str().to_string()]
    }
}

impl Item for Melee {
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

impl Droppable for Melee {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl Buildable for Melee {
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

impl Prime for Melee {
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

impl WikiaLinked for Melee {
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

impl Weapon for Melee {
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

impl MeleeWeapon for Melee {
    fn blocking_angle(&self) -> Option<i64> {
        self.melee.blocking_angle
    }
    fn combo_duration(&self) -> Option<i64> {
        self.melee.combo_duration
    }
    fn follow_through(&self) -> Option<f64> {
        self.melee.follow_through
    }
    fn range(&self) -> Option<f64> {
        self.melee.range
    }
    fn stance_polarity(&self) -> Option<&str> {
        self.melee.stance_polarity.as_ref().map(|p| p.as_str())
    }
    fn slam_attack(&self) -> Option<i64> {
        self.melee.slam_attack
    }
    fn heavy_attack_damage(&self) -> Option<i64> {
        self.melee.heavy_attack_damage
    }
}

impl Equippable for Melee {
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
    fn test_deserialize_melee() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/melee_test.json"
        ));

        let rec: Melee = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Weapons/Grineer/Melee/GrineerTylAxeAndBoar/RegorAxeShield"
        );
        assert_eq!(rec.identity.name, "Ack & Brunt");
        assert_eq!(rec.identity.category, "Melee");
        assert_eq!(rec.type_field, MeleeType::Melee);
        assert!(!rec.trade.tradable);
        assert!(rec.trade.masterable);

        // Weapon stats
        assert!((rec.weapon.critical_chance - 0.2).abs() < 0.01);
        assert!((rec.weapon.total_damage - 149.0).abs() < 0.01);
        assert_eq!(rec.weapon.damage_per_shot.len(), 20);

        // Melee stats
        assert_eq!(rec.melee.blocking_angle, Some(70));

        // Buildable
        assert_eq!(rec.build.build_price, Some(65000));
        assert_eq!(rec.build.components.len(), 5);

        // Equippable
        assert_eq!(rec.equip.slot, Some(Slot::Melee));
        assert!(!rec.prime.is_prime);
    }

    #[test]
    fn test_deserialize_melee_prime() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/melee_test_2.json"
        ));
        let rec: Melee = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Nikana Prime");
        assert!((rec.weapon.total_damage - 198.0).abs() < 0.01);
        assert!((rec.weapon.critical_chance - 0.28).abs() < 0.01);
        assert_eq!(rec.melee.blocking_angle, Some(55));
        assert!(rec.prime.is_prime);
        assert_eq!(rec.prime.vaulted, Some(true));
    }

    #[test]
    fn test_deserialize_melee_base() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/melee_test_3.json"
        ));
        let rec: Melee = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Skana");
        assert!((rec.weapon.total_damage - 120.0).abs() < 0.01);
        assert_eq!(rec.melee.blocking_angle, Some(55));
        assert!(!rec.prime.is_prime);
    }
}

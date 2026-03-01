//! Railjack weapon item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, Patchlog};
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::enums::RailjackType;
use crate::itemdata::props::{
    GunProps, ItemDetailProps, ItemIdentityProps, TradableProps, WeaponProps, WikiaProps,
};
use crate::itemdata::traits::{Droppable, Item, RangedWeapon, Weapon, WikiaLinked};

pub type Root = Vec<Railjack>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Railjack {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: RailjackType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,
    pub product_category: String,
    pub exclude_from_codex: bool,

    pub mastery_req: i64,
    pub slot: i64,

    // Grouped props
    #[serde(flatten)]
    pub weapon: WeaponProps,
    #[serde(flatten)]
    pub gun: GunProps,
    #[serde(flatten)]
    pub wikia: WikiaProps,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Railjack {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["CrewShipWeapons".to_string()]
    }
}

impl Item for Railjack {
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

impl Droppable for Railjack {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl WikiaLinked for Railjack {
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

impl Weapon for Railjack {
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

impl RangedWeapon for Railjack {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_railjack() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/railjack_test.json"
        ));

        let rec: Railjack = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Weapons/CrewShip/MassDriver/AutoCannon/AutoCannonTierA"
        );
        assert_eq!(rec.type_field, RailjackType::RailjackTurret);
        assert!(rec.exclude_from_codex);
    }
}

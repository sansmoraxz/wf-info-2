//! Railjack weapon item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, Introduced, Patchlog};
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::enums::{Noise, RailjackType, Trigger};
use crate::itemdata::traits::{Droppable, Item, WikiaLinked};

pub type Root = Vec<Railjack>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Railjack {
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: RailjackType,
    pub image_name: String,
    pub description: String,
    pub tradable: bool,
    pub masterable: bool,
    pub product_category: String,
    pub exclude_from_codex: bool,

    // Weapon stats
    pub accuracy: f64,
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    pub damage: DamageBreakdown,
    #[serde(default)]
    pub damage_per_shot: Vec<f64>,
    pub fire_rate: f64,
    pub magazine_size: i64,
    pub mastery_req: i64,
    pub multishot: i64,
    pub noise: Noise,
    pub omega_attenuation: f64,
    pub proc_chance: f64,
    pub reload_time: f64,
    pub slot: i64,
    pub total_damage: f64,
    pub trigger: Trigger,

    // Optional weapon fields
    #[serde(default)]
    pub attacks: Vec<Attack>,
    pub disposition: Option<i64>,

    // Wikia
    pub wiki_available: Option<bool>,
    pub wikia_url: Option<String>,
    pub wikia_thumbnail: Option<String>,
    pub introduced: Option<Introduced>,
    pub release_date: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,

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
        &self.unique_name
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn category(&self) -> &str {
        &self.category
    }
    fn type_field(&self) -> &str {
        self.type_field.as_str()
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

impl Droppable for Railjack {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl WikiaLinked for Railjack {
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
            rec.unique_name,
            "/Lotus/Weapons/CrewShip/MassDriver/AutoCannon/AutoCannonTierA"
        );
        assert_eq!(rec.type_field, RailjackType::RailjackTurret);
        assert!(rec.exclude_from_codex);
    }
}

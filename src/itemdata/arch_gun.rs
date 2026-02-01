use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<ArchGun>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchGun {
    pub accuracy: f64,
    #[serde(default)]
    pub attacks: Vec<Attack>,
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub category: String,
    #[serde(default)]
    pub components: Vec<Component>,
    pub consume_on_build: Option<bool>,
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    pub damage: Damage2,
    pub damage_per_shot: Vec<f64>,
    pub description: String,
    pub disposition: Option<i64>,
    pub fire_rate: f64,
    pub image_name: String,
    pub introduced: Option<Introduced>,
    pub is_prime: bool,
    pub magazine_size: i64,
    pub market_cost: Option<i64>,
    pub masterable: bool,
    pub mastery_req: i64,
    pub multishot: i64,
    pub name: String,
    pub noise: String,
    pub omega_attenuation: f64,
    #[serde(default)]
    pub polarities: Vec<String>,
    pub proc_chance: f64,
    pub product_category: String,
    pub release_date: Option<String>,
    pub reload_time: f64,
    pub skip_build_time_price: Option<i64>,
    pub slot: i64,
    #[serde(default)]
    pub tags: Vec<String>,
    pub total_damage: i64,
    pub tradable: bool,
    pub trigger: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub wiki_available: Option<bool>,
    pub wikia_thumbnail: Option<String>,
    pub wikia_url: Option<String>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    pub bp_cost: Option<i64>,
    pub estimated_vault_date: Option<String>,
    pub vaulted: Option<bool>,
    pub max_level_cap: Option<i64>,
}

impl ProductCategory for ArchGun {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.clone()]
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attack {
    pub name: String,
    pub speed: f64,
    pub crit_chance: i64,
    pub crit_mult: f64,
    pub status_chance: f64,
    pub shot_type: Option<String>,
    pub shot_speed: Option<i64>,
    pub flight: Option<i64>,
    pub falloff: Option<Falloff>,
    pub damage: Damage,
    pub charge_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Falloff {
    pub start: i64,
    pub end: f64,
    pub reduction: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Damage {
    pub impact: Option<f64>,
    pub slash: Option<f64>,
    pub puncture: Option<f64>,
    pub blast: Option<i64>,
    pub corrosive: Option<i64>,
    pub gas: Option<i64>,
    pub magnetic: Option<i64>,
    pub radiation: Option<i64>,
    pub viral: Option<i64>,
    pub heat: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub unique_name: String,
    pub name: String,
    pub description: String,
    pub exclude_from_codex: Option<bool>,
    pub item_count: i64,
    pub image_name: String,
    pub tradable: bool,
    pub drops: Vec<Drop>,
    pub masterable: bool,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub prime_selling_price: Option<i64>,
    pub ducats: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop {
    pub chance: f64,
    pub location: String,
    pub rarity: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Damage2 {
    pub total: i64,
    pub impact: f64,
    pub puncture: f64,
    pub slash: f64,
    pub heat: i64,
    pub cold: i64,
    pub electricity: i64,
    pub toxin: i64,
    pub blast: i64,
    pub radiation: i64,
    pub gas: i64,
    pub magnetic: i64,
    pub viral: i64,
    pub corrosive: i64,
    pub void: i64,
    pub tau: i64,
    pub cinematic: i64,
    #[serde(rename = "shieldDrain")]
    pub shield_drain: i64,
    #[serde(rename = "healthDrain")]
    pub health_drain: i64,
    #[serde(rename = "energyDrain")]
    pub energy_drain: i64,
    #[serde(rename = "true")]
    pub true_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Introduced {
    pub name: String,
    pub url: String,
    pub aliases: Vec<String>,
    pub parent: String,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Patchlog {
    pub name: String,
    pub date: String,
    pub url: String,
    pub additions: String,
    pub changes: String,
    pub fixes: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_archgun() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/arch_gun_test.json"
        ));

        let rec: ArchGun = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/Tenno/Archwing/Primary/NokkoArchGun/NokkoArchGun"
        );
    }
}

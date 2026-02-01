use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<Melee>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Melee {
    #[serde(default)]
    pub attacks: Vec<Attack>,
    pub blocking_angle: Option<i64>,
    pub bp_cost: Option<i64>,
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub category: String,
    pub combo_duration: Option<i64>,
    #[serde(default)]
    pub components: Vec<Component>,
    pub consume_on_build: Option<bool>,
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    pub damage: Option<Damage4>,
    pub damage_per_shot: Vec<f64>,
    pub description: String,
    pub disposition: Option<i64>,
    pub fire_rate: f64,
    pub follow_through: Option<f64>,
    pub heavy_attack_damage: Option<i64>,
    pub heavy_slam_attack: Option<i64>,
    pub heavy_slam_radial_damage: Option<i64>,
    pub heavy_slam_radius: Option<i64>,
    pub image_name: String,
    pub introduced: Option<Introduced2>,
    pub is_prime: bool,
    pub market_cost: Option<i64>,
    pub masterable: bool,
    pub mastery_req: i64,
    pub name: String,
    pub omega_attenuation: f64,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    pub proc_chance: f64,
    pub product_category: String,
    pub range: Option<f64>,
    pub release_date: Option<String>,
    pub skip_build_time_price: Option<i64>,
    pub slam_attack: Option<i64>,
    pub slam_radial_damage: Option<i64>,
    pub slam_radius: Option<i64>,
    pub slide_attack: Option<i64>,
    pub slot: i64,
    pub stance_polarity: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub total_damage: f64,
    pub tradable: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub wiki_available: Option<bool>,
    pub wikia_url: Option<String>,
    pub wind_up: Option<f64>,
    #[serde(default)]
    pub polarities: Vec<String>,
    pub estimated_vault_date: Option<String>,
    pub vault_date: Option<String>,
    pub vaulted: Option<bool>,
    #[serde(default)]
    pub drops: Vec<Drop2>,
    pub exclude_from_codex: Option<bool>,
    pub wikia_thumbnail: Option<String>,
    pub max_level_cap: Option<i64>,
    pub item_count: Option<i64>,
    #[serde(default)]
    pub parents: Vec<String>,
}

impl ProductCategory for Melee {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.clone()]
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
    pub name: String,
    pub speed: f64,
    #[serde(rename = "crit_chance")]
    pub crit_chance: f64,
    #[serde(rename = "crit_mult")]
    pub crit_mult: f64,
    #[serde(rename = "status_chance")]
    pub status_chance: f64,
    pub damage: Damage,
    pub slide: Option<String>,
    #[serde(rename = "shot_type")]
    pub shot_type: Option<String>,
    #[serde(rename = "shot_speed")]
    pub shot_speed: Option<i64>,
    pub flight: Option<i64>,
    pub falloff: Option<Falloff>,
    pub slam: Option<Slam>,
    #[serde(rename = "charge_time")]
    pub charge_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub impact: Option<f64>,
    pub slash: Option<f64>,
    pub puncture: Option<f64>,
    pub viral: Option<f64>,
    pub heat: Option<i64>,
    pub blast: Option<i64>,
    pub electricity: Option<i64>,
    pub toxin: Option<f64>,
    pub corrosive: Option<i64>,
    pub radiation: Option<i64>,
    pub gas: Option<i64>,
    pub magnetic: Option<i64>,
    pub cold: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Falloff {
    pub start: i64,
    pub end: f64,
    pub reduction: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slam {
    pub damage: String,
    pub radial: Radial,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Radial {
    pub damage: String,
    pub radius: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub unique_name: String,
    pub name: String,
    pub description: String,
    pub item_count: i64,
    pub image_name: String,
    pub tradable: bool,
    pub masterable: bool,
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub prime_selling_price: Option<i64>,
    pub ducats: Option<i64>,
    #[serde(default)]
    pub damage_per_shot: Vec<f64>,
    pub total_damage: Option<f64>,
    pub critical_chance: Option<f64>,
    pub critical_multiplier: Option<f64>,
    pub proc_chance: Option<f64>,
    pub fire_rate: Option<f64>,
    pub mastery_req: Option<i64>,
    pub product_category: Option<String>,
    pub slot: Option<i64>,
    pub accuracy: Option<f64>,
    pub omega_attenuation: Option<f64>,
    pub noise: Option<String>,
    pub trigger: Option<String>,
    pub magazine_size: Option<i64>,
    pub reload_time: Option<f64>,
    pub multishot: Option<i64>,
    pub damage: Option<Damage2>,
    pub wiki_available: Option<bool>,
    #[serde(default)]
    pub attacks: Vec<Attack2>,
    pub market_cost: Option<i64>,
    pub bp_cost: Option<i64>,
    #[serde(default)]
    pub polarities: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub wikia_thumbnail: Option<String>,
    pub wikia_url: Option<String>,
    pub disposition: Option<i64>,
    pub introduced: Option<Introduced>,
    pub release_date: Option<String>,
    pub blocking_angle: Option<i64>,
    pub combo_duration: Option<i64>,
    pub follow_through: Option<f64>,
    pub range: Option<f64>,
    pub slam_attack: Option<i64>,
    pub slam_radial_damage: Option<i64>,
    pub slam_radius: Option<i64>,
    pub slide_attack: Option<i64>,
    pub heavy_attack_damage: Option<i64>,
    pub heavy_slam_attack: Option<i64>,
    pub heavy_slam_radial_damage: Option<i64>,
    pub heavy_slam_radius: Option<i64>,
    pub wind_up: Option<f64>,
    pub stance_polarity: Option<String>,
    pub exclude_from_codex: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drop {
    pub chance: f64,
    pub location: String,
    pub rarity: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage2 {
    pub total: f64,
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
    pub shield_drain: i64,
    pub health_drain: i64,
    pub energy_drain: i64,
    #[serde(rename = "true")]
    pub true_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attack2 {
    pub name: String,
    pub speed: f64,
    pub crit_chance: f64,
    pub crit_mult: f64,
    pub status_chance: f64,
    pub shot_type: Option<String>,
    pub shot_speed: Option<i64>,
    pub flight: Option<i64>,
    pub damage: Damage3,
    pub slide: Option<String>,
    pub falloff: Option<Falloff2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Damage3 {
    pub impact: Option<f64>,
    pub slash: Option<f64>,
    pub puncture: Option<f64>,
    pub blast: Option<i64>,
    pub magnetic: Option<i64>,
    pub cold: Option<i64>,
    pub electricity: Option<i64>,
    pub heat: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Falloff2 {
    pub start: i64,
    pub end: f64,
    pub reduction: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Introduced {
    pub name: String,
    pub url: String,
    pub aliases: Vec<String>,
    pub parent: String,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage4 {
    pub total: f64,
    pub impact: f64,
    pub puncture: f64,
    pub slash: f64,
    pub heat: i64,
    pub cold: i64,
    pub electricity: i64,
    pub toxin: f64,
    pub blast: i64,
    pub radiation: i64,
    pub gas: i64,
    pub magnetic: i64,
    pub viral: i64,
    pub corrosive: i64,
    pub void: i64,
    pub tau: i64,
    pub cinematic: i64,
    pub shield_drain: i64,
    pub health_drain: i64,
    pub energy_drain: i64,
    #[serde(rename = "true")]
    pub true_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Introduced2 {
    pub name: String,
    pub url: String,
    pub aliases: Vec<String>,
    pub parent: String,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patchlog {
    pub name: String,
    pub date: String,
    pub url: String,
    pub additions: String,
    pub changes: String,
    pub fixes: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drop2 {
    pub chance: f64,
    pub location: String,
    pub rarity: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_archmelee() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/melee_test.json"
        ));

        let rec: Melee = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/Grineer/Melee/GrineerTylAxeAndBoar/RegorAxeShield"
        );
    }
}

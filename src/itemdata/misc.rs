use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Root = Vec<Misc>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Misc {
    pub category: String,
    pub description: Option<String>,
    pub exclude_from_codex: Option<bool>,
    pub image_name: Option<String>,
    pub masterable: bool,
    pub name: String,
    pub tradable: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub show_in_inventory: Option<bool>,
    pub required: Option<i64>,
    pub standing: Option<i64>,
    pub item_count: Option<i64>,
    pub probability: Option<f64>,
    pub rarity: Option<String>,
    pub reward_name: Option<String>,
    pub tier: Option<i64>,
    pub fusion_points: Option<i64>,
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub parents: Vec<String>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    #[serde(default)]
    pub components: Vec<Component>,
    pub consume_on_build: Option<bool>,
    pub critical_chance: Option<f64>,
    pub critical_multiplier: Option<f64>,
    #[serde(default)]
    pub damage_per_shot: Vec<f64>,
    pub fire_rate: Option<f64>,
    pub mastery_req: Option<i64>,
    pub omega_attenuation: Option<f64>,
    pub proc_chance: Option<f64>,
    pub product_category: Option<String>,
    pub skip_build_time_price: Option<i64>,
    pub total_damage: Option<f64>,
    pub accuracy: Option<f64>,
    #[serde(default)]
    pub attacks: Vec<Attack>,
    pub damage: Option<Damage2>,
    pub disposition: Option<i64>,
    pub introduced: Option<Introduced>,
    pub multishot: Option<i64>,
    pub noise: Option<String>,
    #[serde(default)]
    pub polarities: Vec<String>,
    pub release_date: Option<String>,
    pub reload_time: Option<f64>,
    pub slot: Option<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub trigger: Option<String>,
    pub wiki_available: Option<bool>,
    pub wikia_thumbnail: Option<String>,
    pub wikia_url: Option<String>,
    pub vaulted: Option<bool>,
    pub prime_omega_attenuation: Option<f64>,
    pub blocking_angle: Option<i64>,
    pub combo_duration: Option<i64>,
    pub follow_through: Option<f64>,
    pub heavy_attack_damage: Option<i64>,
    pub heavy_slam_attack: Option<i64>,
    pub heavy_slam_radial_damage: Option<i64>,
    pub heavy_slam_radius: Option<i64>,
    pub range: Option<f64>,
    pub slam_attack: Option<i64>,
    pub slam_radial_damage: Option<i64>,
    pub slam_radius: Option<i64>,
    pub slide_attack: Option<i64>,
    pub stance_polarity: Option<String>,
    pub wind_up: Option<f64>,
    pub magazine_size: Option<i64>,
    pub bin_capacity: Option<i64>,
    pub bin_count: Option<i64>,
    pub capacity_multiplier: Option<Vec<i64>>,
    pub durability: Option<i64>,
    pub fill_rate: Option<f64>,
    pub repair_rate: Option<i64>,
    #[serde(default)]
    pub specialities: Vec<Value>,
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
pub struct Component {
    pub unique_name: String,
    pub name: String,
    pub description: String,
    pub item_count: i64,
    pub image_name: String,
    pub tradable: bool,
    pub masterable: bool,
    #[serde(default)]
    pub drops: Vec<Drop2>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
    pub name: String,
    pub speed: Option<f64>,
    #[serde(rename = "crit_chance")]
    pub crit_chance: i64,
    #[serde(rename = "crit_mult")]
    pub crit_mult: f64,
    #[serde(rename = "status_chance")]
    pub status_chance: i64,
    pub damage: Damage,
    pub slide: Option<String>,
    pub slam: Option<Slam>,
    #[serde(rename = "shot_type")]
    pub shot_type: Option<String>,
    #[serde(rename = "shot_speed")]
    pub shot_speed: Option<i64>,
    pub flight: Option<i64>,
    pub falloff: Option<Falloff>,
    #[serde(rename = "charge_time")]
    pub charge_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub impact: Option<f64>,
    pub slash: Option<f64>,
    pub puncture: Option<f64>,
    pub void: Option<i64>,
    pub blast: Option<i64>,
    pub heat: Option<i64>,
    pub electricity: Option<i64>,
    pub cold: Option<i64>,
    pub radiation: Option<i64>,
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
    pub radius: i64,
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
#[serde(rename_all = "camelCase")]
pub struct Introduced {
    pub name: String,
    pub url: String,
    pub aliases: Vec<String>,
    pub parent: String,
    pub date: String,
}

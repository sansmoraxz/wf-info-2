use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<ArchMelee>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchMelee {
    pub attacks: Vec<Attack>,
    pub blocking_angle: i64,
    pub bp_cost: Option<i64>,
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub category: String,
    pub combo_duration: i64,
    #[serde(default)]
    pub components: Vec<Component>,
    pub consume_on_build: Option<bool>,
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    pub damage: Damage4,
    pub damage_per_shot: Vec<f64>,
    pub description: String,
    pub disposition: i64,
    pub fire_rate: f64,
    pub follow_through: f64,
    pub heavy_attack_damage: i64,
    pub heavy_slam_attack: i64,
    pub image_name: String,
    pub introduced: Introduced2,
    pub is_prime: bool,
    pub market_cost: Option<i64>,
    pub masterable: bool,
    pub mastery_req: i64,
    pub name: String,
    pub omega_attenuation: f64,
    pub polarities: Vec<String>,
    pub proc_chance: f64,
    pub product_category: String,
    pub range: f64,
    pub release_date: String,
    pub skip_build_time_price: Option<i64>,
    pub slam_attack: i64,
    pub slam_radial_damage: i64,
    pub slam_radius: i64,
    pub slide_attack: i64,
    pub slot: i64,
    pub tags: Vec<String>,
    pub total_damage: f64,
    pub tradable: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub wiki_available: bool,
    pub wikia_thumbnail: String,
    pub wikia_url: String,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for ArchMelee {
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
    pub status_chance: i64,
    pub damage: Damage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Damage {
    pub impact: f64,
    pub slash: f64,
    pub puncture: f64,
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
    pub drops: Vec<Drop>,
    pub masterable: bool,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(default)]
    pub damage_per_shot: Vec<f64>,
    pub total_damage: Option<i64>,
    pub critical_chance: Option<f64>,
    pub critical_multiplier: Option<f64>,
    pub proc_chance: Option<f64>,
    pub fire_rate: Option<i64>,
    pub mastery_req: Option<i64>,
    pub product_category: Option<String>,
    pub slot: Option<i64>,
    pub omega_attenuation: Option<f64>,
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
    pub damage: Option<Damage2>,
    pub wiki_available: Option<bool>,
    pub attacks: Option<Vec<Attack2>>,
    pub market_cost: Option<i64>,
    pub bp_cost: Option<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub stance_polarity: Option<String>,
    pub wikia_thumbnail: Option<String>,
    pub wikia_url: Option<String>,
    pub disposition: Option<i64>,
    pub introduced: Option<Introduced>,
    pub release_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop {
    pub chance: i64,
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
pub struct Attack2 {
    pub name: String,
    pub speed: i64,
    pub crit_chance: i64,
    pub crit_mult: f64,
    pub status_chance: i64,
    pub damage: Damage3,
    pub slide: Option<String>,
    pub shot_type: Option<String>,
    pub falloff: Option<Falloff>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Damage3 {
    pub impact: Option<f64>,
    pub slash: Option<f64>,
    pub puncture: Option<f64>,
    pub blast: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Falloff {
    pub start: i64,
    pub end: i64,
    pub reduction: f64,
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
#[serde(rename_all = "camelCase")]
pub struct Damage4 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_archmelee() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/arch_melee_test.json"
        ));

        let rec: ArchMelee = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/Tenno/Archwing/Melee/ArchScythe/ArchScythe"
        );
    }
}

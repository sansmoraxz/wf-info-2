use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<Primary>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Primary {
    pub accuracy: f64,
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
    pub damage: Damage4,
    pub damage_per_shot: Vec<f64>,
    pub description: String,
    pub disposition: i64,
    pub fire_rate: f64,
    pub image_name: String,
    pub introduced: Introduced2,
    pub is_prime: bool,
    pub magazine_size: Option<i64>,
    pub market_cost: Option<i64>,
    pub masterable: bool,
    pub mastery_req: i64,
    pub multishot: i64,
    pub name: String,
    pub noise: String,
    pub omega_attenuation: f64,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    #[serde(default)]
    pub polarities: Vec<String>,
    pub proc_chance: f64,
    pub product_category: String,
    pub release_date: String,
    pub reload_time: f64,
    pub skip_build_time_price: Option<i64>,
    pub slot: i64,
    pub tags: Vec<String>,
    pub total_damage: f64,
    pub tradable: bool,
    pub trigger: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub wiki_available: bool,
    pub wikia_thumbnail: String,
    pub wikia_url: String,
    pub vaulted: Option<bool>,
    pub bp_cost: Option<i64>,
    pub estimated_vault_date: Option<String>,
    pub vault_date: Option<String>,
    pub max_level_cap: Option<i64>,
    pub item_count: Option<i64>,
    #[serde(default)]
    pub parents: Vec<String>,
    #[serde(default)]
    pub drops: Vec<Drop2>,
}

impl ProductCategory for Primary {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.clone()]
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
    pub name: String,
    pub speed: Option<f64>,
    #[serde(rename = "crit_chance")]
    pub crit_chance: f64,
    #[serde(rename = "crit_mult")]
    pub crit_mult: f64,
    #[serde(rename = "status_chance")]
    pub status_chance: f64,
    #[serde(rename = "shot_type")]
    pub shot_type: Option<String>,
    #[serde(rename = "shot_speed")]
    pub shot_speed: Option<i64>,
    pub flight: Option<i64>,
    pub damage: Damage,
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
    pub radiation: Option<i64>,
    pub blast: Option<i64>,
    pub electricity: Option<i64>,
    pub heat: Option<i64>,
    pub magnetic: Option<i64>,
    pub toxin: Option<i64>,
    pub viral: Option<f64>,
    pub corrosive: Option<i64>,
    pub cold: Option<i64>,
    pub void: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Falloff {
    pub start: f64,
    pub end: f64,
    pub reduction: f64,
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
    pub drops: Vec<Drop>,
    pub prime_selling_price: Option<i64>,
    pub ducats: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(default)]
    pub damage_per_shot: Vec<f64>,
    pub total_damage: Option<i64>,
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
    pub shield_drain: i64,
    pub health_drain: i64,
    pub energy_drain: i64,
    #[serde(rename = "true")]
    pub true_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack2 {
    pub name: String,
    pub speed: f64,
    #[serde(rename = "crit_chance")]
    pub crit_chance: f64,
    #[serde(rename = "crit_mult")]
    pub crit_mult: f64,
    #[serde(rename = "status_chance")]
    pub status_chance: f64,
    #[serde(rename = "shot_type")]
    pub shot_type: String,
    #[serde(rename = "shot_speed")]
    pub shot_speed: Option<i64>,
    pub flight: Option<i64>,
    pub damage: Damage3,
    #[serde(rename = "charge_time")]
    pub charge_time: Option<f64>,
    pub falloff: Option<Falloff2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage3 {
    pub impact: Option<f64>,
    pub slash: Option<f64>,
    pub puncture: Option<f64>,
    pub heat: Option<i64>,
    pub toxin: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub toxin: i64,
    pub blast: f64,
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
    pub chance: i64,
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
    fn test_deserialize_primary() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/primary_test.json"
        ));

        let rec: Primary = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/Tenno/LongGuns/SapientPrimary/SapientPrimaryWeapon"
        );
    }
}

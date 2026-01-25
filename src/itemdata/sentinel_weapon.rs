use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<SentinelWeapon>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentinelWeapon {
    pub attacks: Vec<Attack>,
    pub blocking_angle: Option<i64>,
    pub category: String,
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    pub damage: Damage2,
    pub damage_per_shot: Vec<f64>,
    pub description: String,
    pub disposition: i64,
    pub exclude_from_codex: Option<bool>,
    pub fire_rate: f64,
    pub image_name: String,
    pub introduced: Introduced,
    pub is_prime: bool,
    pub masterable: bool,
    pub mastery_req: i64,
    pub name: String,
    pub omega_attenuation: f64,
    pub proc_chance: f64,
    pub product_category: String,
    pub release_date: String,
    pub sentinel: bool,
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
    pub accuracy: Option<f64>,
    pub magazine_size: Option<i64>,
    pub multishot: Option<i64>,
    pub noise: Option<String>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    pub reload_time: Option<f64>,
    pub trigger: Option<String>,
    pub vaulted: Option<bool>,
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub components: Option<Vec<Component>>,
    pub consume_on_build: Option<bool>,
    #[serde(default)]
    pub polarities: Vec<String>,
    pub skip_build_time_price: Option<i64>,
}

impl ProductCategory for SentinelWeapon {
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
    #[serde(rename = "shot_type")]
    pub shot_type: Option<String>,
    #[serde(rename = "shot_speed")]
    pub shot_speed: Option<i64>,
    pub flight: Option<i64>,
    pub falloff: Option<Falloff>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub slash: Option<f64>,
    pub puncture: Option<f64>,
    pub cold: Option<i64>,
    pub impact: Option<f64>,
    pub heat: Option<i64>,
    pub blast: Option<i64>,
    pub toxin: Option<i64>,
    pub electricity: Option<i64>,
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
    pub drops: Vec<Drop>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drop {
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
    fn test_deserialize_sentinel_weapon() {
        let json_data = r#"
{
  "attacks": [
    {
      "name": "Normal Attack",
      "speed": 1,
      "crit_chance": 10,
      "crit_mult": 3,
      "status_chance": 15,
      "damage": {
        "impact": 150,
        "puncture": 150
      }
    }
  ],
  "blockingAngle": 90,
  "category": "Primary",
  "criticalChance": 0.1,
  "criticalMultiplier": 3,
  "damage": {
    "total": 300,
    "impact": 150,
    "puncture": 0,
    "slash": 150,
    "heat": 0,
    "cold": 0,
    "electricity": 0,
    "toxin": 0,
    "blast": 0,
    "radiation": 0,
    "gas": 0,
    "magnetic": 0,
    "viral": 0,
    "corrosive": 0,
    "void": 0,
    "tau": 0,
    "cinematic": 0,
    "shieldDrain": 0,
    "healthDrain": 0,
    "energyDrain": 0,
    "true": 0
  },
  "damagePerShot": [
    150,
    150,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0
  ],
  "description": "Hound melee attacks pierce hard with <DT_IMPACT_COLOR>Impact and <DT_PUNCTURE_COLOR>Puncture Damage.",
  "disposition": 3,
  "excludeFromCodex": true,
  "fireRate": 1,
  "imageName": "batoten-a5990aba68.png",
  "introduced": {
    "name": "Update 30.5",
    "url": "https://wiki.warframe.com/w/Update_30%23Update_30.5",
    "aliases": [
      "30.5"
    ],
    "parent": "30.5",
    "date": "2021-07-06"
  },
  "isPrime": false,
  "masterable": true,
  "masteryReq": 0,
  "name": "Batoten",
  "omegaAttenuation": 1,
  "procChance": 0.14999998,
  "productCategory": "SentinelWeapons",
  "releaseDate": "2021-07-06",
  "sentinel": true,
  "slot": 1,
  "tags": [],
  "totalDamage": 300,
  "tradable": false,
  "type": "Companion Weapon",
  "uniqueName": "/Lotus/Types/Friendly/Pets/ZanukaPets/ZanukaPetMeleeWeaponIP",
  "wikiAvailable": true,
  "wikiaThumbnail": "https://wiki.warframe.com/images/Batoten.png?8d059",
  "wikiaUrl": "https://wiki.warframe.com/w/Batoten"
}
"#;

        let rec: SentinelWeapon = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Friendly/Pets/ZanukaPets/ZanukaPetMeleeWeaponIP"
        );
    }
}

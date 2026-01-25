use serde::{Deserialize, Serialize};

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
        let json_data = r#"
{
  "attacks": [
    {
      "name": "Normal Attack",
      "speed": 1,
      "crit_chance": 15,
      "crit_mult": 2,
      "status_chance": 20,
      "damage": {
        "impact": 19.7,
        "slash": 58.7,
        "puncture": 313.6
      }
    }
  ],
  "blockingAngle": 90,
  "bpCost": 35000,
  "buildPrice": 25000,
  "buildQuantity": 1,
  "buildTime": 43200,
  "category": "Arch-Melee",
  "comboDuration": 5,
  "components": [
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/WeaponParts/ArchScytheBlade",
      "name": "Blade",
      "description": "An Archwing weapon-crafting component.",
      "itemCount": 1,
      "imageName": "kaszas-blade.png",
      "tradable": false,
      "drops": [
        {
          "chance": 1,
          "location": "Red Veil, Honored",
          "rarity": "Common",
          "type": "Kaszas Blade"
        }
      ],
      "masterable": false
    },
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/ArchScytheBlueprint",
      "name": "Blueprint",
      "description": "Become an angel of death, with this Archwing scythe.",
      "itemCount": 1,
      "imageName": "blueprint.png",
      "tradable": false,
      "masterable": false,
      "drops": []
    },
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/WeaponParts/ArchScytheHandle",
      "name": "Handle",
      "description": "An Archwing weapon-crafting component.",
      "itemCount": 1,
      "imageName": "kaszas-handle.png",
      "tradable": false,
      "drops": [
        {
          "chance": 1,
          "location": "Steel Meridian, Valiant",
          "rarity": "Common",
          "type": "Kaszas Handle"
        }
      ],
      "masterable": false
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/Tellurium",
      "name": "Tellurium",
      "description": "This rare metal is foreign to the Origin System and can only be found in asteroids that have made the long journey from other stars.\n\nLocation: Archwing Missions on Uranus",
      "itemCount": 3,
      "imageName": "tellurium-9236306b55.png",
      "tradable": false,
      "drops": [],
      "masterable": false,
      "type": "Resource"
    }
  ],
  "consumeOnBuild": true,
  "criticalChance": 0.15000001,
  "criticalMultiplier": 2,
  "damage": {
    "total": 392,
    "impact": 19.6,
    "puncture": 58.799992,
    "slash": 313.60001,
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
    19.6,
    313.60001,
    58.799992,
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
  "description": "Become an angel of death, with this Archwing scythe.",
  "disposition": 1,
  "fireRate": 1,
  "followThrough": 0.69999999,
  "heavyAttackDamage": 392,
  "heavySlamAttack": 392,
  "imageName": "kaszas-fa004e9997.png",
  "introduced": {
    "name": "Update 17.5",
    "url": "https://wiki.warframe.com/w/Update_17%23Update_17.5",
    "aliases": [
      "17.5.0",
      "17.5"
    ],
    "parent": "17.5",
    "date": "2015-10-01"
  },
  "isPrime": false,
  "marketCost": 150,
  "masterable": true,
  "masteryReq": 4,
  "name": "Kaszas",
  "omegaAttenuation": 0.5,
  "patchlogs": [
    {
      "name": "Update 30.5: Sisters of Parvos",
      "date": "2021-07-06T15:02:10Z",
      "url": "https://forums.warframe.com/topic/1269749-update-305-sisters-of-parvos/",
      "additions": "Added custom spin whoosh sounds and updated animations for the Kaszas Arch-Melee weapon. ",
      "changes": "",
      "fixes": ""
    },
    {
      "name": "Hotfix 18.8.2",
      "date": "2016-04-13T18:53:32Z",
      "url": "https://forums.warframe.com/topic/637581-hotfix-1882/",
      "additions": "",
      "changes": "",
      "fixes": "•    Fixed the Kaszas not having its unique combo attacks."
    },
    {
      "name": "Hotfix 17.6.1",
      "date": "2015-10-07T21:04:29Z",
      "url": "https://forums.warframe.com/topic/540998-hotfix-1761/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Kaszas not properly displaying channeling FX."
    },
    {
      "name": "Update 17.5: The Jordas Precept. + Hotfix 17.5.1",
      "date": "2015-10-02T02:05:33Z",
      "url": "https://forums.warframe.com/topic/536207-update-175-the-jordas-precept-hotfix-1751/",
      "additions": "Kaszas: Become an angel of death with this Archwing scythe.",
      "changes": "",
      "fixes": ""
    }
  ],
  "polarities": [
    "vazarin"
  ],
  "procChance": 0.19999999,
  "productCategory": "SpaceMelee",
  "range": 2.5,
  "releaseDate": "2015-10-01",
  "skipBuildTimePrice": 50,
  "slamAttack": 392,
  "slamRadialDamage": 784,
  "slamRadius": 3,
  "slideAttack": 784,
  "slot": 5,
  "tags": [
    "Tenno"
  ],
  "totalDamage": 392,
  "tradable": false,
  "type": "Arch-Melee",
  "uniqueName": "/Lotus/Weapons/Tenno/Archwing/Melee/ArchScythe/ArchScythe",
  "wikiAvailable": true,
  "wikiaThumbnail": "https://wiki.warframe.com/images/Kaszas.png?9fcb4",
  "wikiaUrl": "https://wiki.warframe.com/w/Kaszas"
}
"#;

        let rec: ArchMelee = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/Tenno/Archwing/Melee/ArchScythe/ArchScythe"
        );
    }
}

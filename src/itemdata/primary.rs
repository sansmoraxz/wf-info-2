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
        let json_data = r#"
{
  "accuracy": 23.529411,
  "attacks": [
    {
      "name": "Rocket Impact",
      "speed": 12,
      "crit_chance": 32,
      "crit_mult": 2.8,
      "status_chance": 6,
      "shot_type": "Projectile",
      "shot_speed": 70,
      "flight": 70,
      "damage": {
        "impact": 35
      }
    },
    {
      "name": "Rocket Explosion",
      "speed": 12,
      "crit_chance": 32,
      "crit_mult": 2.8,
      "status_chance": 6,
      "shot_type": "AoE",
      "falloff": {
        "start": 0,
        "end": 4,
        "reduction": 0.5
      },
      "damage": {
        "slash": 8.8,
        "puncture": 35.2
      }
    }
  ],
  "buildPrice": 25000,
  "buildQuantity": 1,
  "buildTime": 86400,
  "category": "Primary",
  "components": [
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/SapientPrimaryBlueprint",
      "name": "Blueprint",
      "description": "Using a barrage of rapid-fire plasma rockets, Gauss’ signature weapon lays down a path of destruction. Reloads are faster while sprinting, even more so in Gauss’ hands. For safety, rockets arm after traveling a safe distance.",
      "itemCount": 1,
      "imageName": "blueprint.png",
      "tradable": false,
      "masterable": false,
      "drops": [
        {
          "chance": 0.0125,
          "location": "Demolisher Boiler",
          "rarity": "Common",
          "type": "Acceltra Blueprint"
        },
        {
          "chance": 0.0125,
          "location": "Demolisher Charger",
          "rarity": "Common",
          "type": "Acceltra Blueprint"
        },
        {
          "chance": 0.0125,
          "location": "Demolisher Juggernaut",
          "rarity": "Common",
          "type": "Acceltra Blueprint"
        },
        {
          "chance": 0.0125,
          "location": "Demolisher Thrasher",
          "rarity": "Common",
          "type": "Acceltra Blueprint"
        }
      ]
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/ConcentratedGas",
      "name": "Hexenon",
      "description": "A reagent commonly used to produce highly combustible fuel.\n\nLocation: Jupiter",
      "itemCount": 200,
      "imageName": "hexenon-cd08831fa0.png",
      "tradable": false,
      "drops": [],
      "masterable": false,
      "type": "Resource"
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/Nanospores",
      "name": "Nano Spores",
      "description": "Fibrous technocyte tumour. Handle Infested tissue with caution.\n\nLocation: Saturn, Neptune, Eris, and Deimos",
      "itemCount": 8000,
      "imageName": "nano-spores-8933019524.png",
      "tradable": false,
      "drops": [],
      "masterable": false,
      "type": "Resource"
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/Neurode",
      "name": "Neurodes",
      "description": "Biotech sensor organ harvested from Infested entities.\n\nLocation: Earth, Lua, Eris, and Deimos",
      "itemCount": 4,
      "imageName": "neurodes-c027fd4a28.png",
      "tradable": false,
      "drops": [],
      "masterable": false
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/Plastids",
      "name": "Plastids",
      "description": "A disgusting nanite-infested tissue mass.\n\nLocation: Phobos, Saturn, Uranus, Pluto, and Eris",
      "itemCount": 925,
      "imageName": "plastids-dabf813edd.png",
      "tradable": false,
      "drops": [],
      "masterable": false,
      "type": "Resource"
    }
  ],
  "consumeOnBuild": true,
  "criticalChance": 0.31999999,
  "criticalMultiplier": 2.8,
  "damage": {
    "total": 70,
    "impact": 26,
    "puncture": 8.7999992,
    "slash": 35.200001,
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
    26,
    35.200001,
    8.7999992,
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
  "description": "Using a barrage of rapid-fire plasma rockets, Gauss’ signature weapon lays down a path of destruction. Reloads are faster while sprinting, even more so in Gauss’ hands. For safety, rockets arm after traveling a safe distance.",
  "disposition": 1,
  "fireRate": 12.000001,
  "imageName": "acceltra-f87797699a.png",
  "introduced": {
    "name": "Update 25.7",
    "url": "https://wiki.warframe.com/w/Update_25%23Update_25.7",
    "aliases": [
      "25.7",
      "Saint of Altra"
    ],
    "parent": "25.7",
    "date": "2019-08-29"
  },
  "isPrime": false,
  "magazineSize": 48,
  "marketCost": 240,
  "masterable": true,
  "masteryReq": 8,
  "multishot": 1,
  "name": "Acceltra",
  "noise": "Alarming",
  "omegaAttenuation": 0.64999998,
  "patchlogs": [
    {
      "name": "Veilbreaker: Update 32",
      "date": "2022-09-07T15:00:11Z",
      "url": "https://forums.warframe.com/topic/1321162-veilbreaker-update-32/",
      "additions": "",
      "changes": "Acceltra",
      "fixes": ""
    },
    {
      "name": "Update 31.1.0: Echoes of War",
      "date": "2022-02-09T15:59:43Z",
      "url": "https://forums.warframe.com/topic/1299619-update-3110-echoes-of-war/",
      "additions": "",
      "changes": "Solstice Acceltra Skin",
      "fixes": ""
    },
    {
      "name": "Nights of Naberus: Hotfix 30.8.1",
      "date": "2021-10-06T19:47:19Z",
      "url": "https://forums.warframe.com/topic/1283279-nights-of-naberus-hotfix-3081/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed excessively bright explosion FX from the Shedu and Acceltra."
    },
    {
      "name": "Operation: Scarlet Spear: TennoGen 27.3.8 + 27.3.8.1 + 27.3.8.2",
      "date": "2020-04-02T16:33:08Z",
      "url": "https://forums.warframe.com/topic/1181462-operation-scarlet-spear-tennogen-2738-27381-27382/",
      "additions": "",
      "changes": "The Pox, Acceltra, and Shedu have been given explosion FX tweaks for visual and performance improvement.",
      "fixes": ""
    },
    {
      "name": "Warframe Revised: Hotfix 27.2.2",
      "date": "2020-03-06T20:01:27Z",
      "url": "https://forums.warframe.com/topic/1173118-warframe-revised-hotfix-2722/",
      "additions": "",
      "changes": "Acceltra: 50%",
      "fixes": ""
    },
    {
      "name": "Empyrean: Update 27",
      "date": "2019-12-13T02:34:29Z",
      "url": "https://forums.warframe.com/topic/1151428-empyrean-update-27/",
      "additions": "",
      "changes": "Acceltra: 1->0.8",
      "fixes": ""
    },
    {
      "name": "Rising Tide: Hotfix 26.1.3",
      "date": "2019-11-26T19:33:56Z",
      "url": "https://forums.warframe.com/topic/1146782-rising-tide-hotfix-2613/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Acceltra sounds being heard from other players."
    },
    {
      "name": "The Old Blood: Hotfix 26.0.7",
      "date": "2019-11-14T21:23:52Z",
      "url": "https://forums.warframe.com/topic/1142475-the-old-blood-hotfix-2607/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed missing hit notification sounds on the Acceltra. "
    },
    {
      "name": "Arbitrations Revisited: Hotfix 25.7.6",
      "date": "2019-09-18T19:13:08Z",
      "url": "https://forums.warframe.com/topic/1129207-arbitrations-revisited-hotfix-2576/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed unintended fire rate increase for Acceltra while jumping. "
    },
    {
      "name": "Prime Vault: Hotfix 25.7.4 + 25.7.4.1",
      "date": "2019-09-05T19:02:06Z",
      "url": "https://forums.warframe.com/topic/1126459-prime-vault-hotfix-2574-25741/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Acceltra not having the correct model/text when picking it up."
    },
    {
      "name": "Hotfix 25.7.2 + 25.7.2.1",
      "date": "2019-08-30T19:11:49Z",
      "url": "https://forums.warframe.com/topic/1124402-hotfix-2572-25721/",
      "additions": "",
      "changes": "Acceltra can now equip Rifle Ammo Mutation Mod.",
      "fixes": ""
    },
    {
      "name": "Saint of Altra: Update 25.7.0",
      "date": "2019-08-29T21:35:56Z",
      "url": "https://forums.warframe.com/topic/1123841-saint-of-altra-update-2570/",
      "additions": "",
      "changes": "",
      "fixes": "Accelerate past the redline with Gauss. This collection includes the Gauss Warframe, Gauss Mag Helmet, Acceltra rapid-fire rocket launcher, and Akarius secondary rocket launchers."
    }
  ],
  "polarities": [
    "naramon"
  ],
  "procChance": 0.060000002,
  "productCategory": "LongGuns",
  "releaseDate": "2019-08-29",
  "reloadTime": 2,
  "skipBuildTimePrice": 35,
  "slot": 1,
  "tags": [
    "Tenno"
  ],
  "totalDamage": 70,
  "tradable": false,
  "trigger": "Auto",
  "type": "Rifle",
  "uniqueName": "/Lotus/Weapons/Tenno/LongGuns/SapientPrimary/SapientPrimaryWeapon",
  "wikiAvailable": true,
  "wikiaThumbnail": "https://wiki.warframe.com/images/Acceltra.png?d00f4",
  "wikiaUrl": "https://wiki.warframe.com/w/Acceltra"
}
"#;

        let rec: Primary = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/Tenno/LongGuns/SapientPrimary/SapientPrimaryWeapon"
        );
    }
}

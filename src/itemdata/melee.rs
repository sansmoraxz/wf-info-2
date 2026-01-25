use serde::{Deserialize, Serialize};

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
        let json_data = r#"
{
  "attacks": [
    {
      "name": "Normal Attack",
      "speed": 0.833,
      "crit_chance": 20,
      "crit_mult": 2,
      "status_chance": 10,
      "damage": {
        "impact": 14.9,
        "slash": 119.2,
        "puncture": 14.9
      },
      "slide": "149"
    },
    {
      "name": "Spectral Axe",
      "speed": 0.833,
      "crit_chance": 13,
      "crit_mult": 2,
      "status_chance": 33,
      "shot_type": "Projectile",
      "shot_speed": 60,
      "flight": 60,
      "damage": {
        "impact": 14.9,
        "slash": 119.2,
        "puncture": 14.9
      }
    },
    {
      "name": "Spectral Axe Explosion",
      "speed": 0.833,
      "crit_chance": 13,
      "crit_mult": 2,
      "status_chance": 33,
      "shot_type": "AoE",
      "falloff": {
        "start": 0,
        "end": 5,
        "reduction": 0.3
      },
      "damage": {
        "heat": 303
      }
    },
    {
      "name": "Slam Attack",
      "speed": 0.833,
      "crit_chance": 20,
      "crit_mult": 2,
      "status_chance": 10,
      "shot_type": "AoE",
      "falloff": {
        "start": 0,
        "end": 7,
        "reduction": 0.5
      },
      "damage": {
        "impact": 298
      }
    },
    {
      "name": "Heavy Slam Attack",
      "speed": 0.833,
      "crit_chance": 20,
      "crit_mult": 2,
      "status_chance": 10,
      "shot_type": "AoE",
      "falloff": {
        "start": 0,
        "end": 8,
        "reduction": 0.3
      },
      "damage": {
        "blast": 447
      }
    }
  ],
  "blockingAngle": 70,
  "bpCost": 15000,
  "buildPrice": 65000,
  "buildQuantity": 1,
  "buildTime": 86400,
  "category": "Melee",
  "comboDuration": 5,
  "components": [
    {
      "uniqueName": "/Lotus/Weapons/ClanTech/Chemical/RegorAxeShieldBlueprint",
      "name": "Blueprint",
      "description": "Tyl Regor’s custom axe and shield are how he likes to eliminate ‘frustrations’.",
      "itemCount": 1,
      "imageName": "blueprint.png",
      "tradable": false,
      "masterable": false,
      "drops": []
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/Cryotic",
      "name": "Cryotic",
      "description": "Usually found in extreme sub-zero environments, Cryotic instantly freezes anything it contacts.\n\nLocation: Excavation Missions on Venus, Earth, Phobos, Europa, and Neptune",
      "itemCount": 3300,
      "imageName": "cryotic-0c63ca0f8d.png",
      "tradable": false,
      "drops": [],
      "masterable": false,
      "type": "Resource"
    },
    {
      "uniqueName": "/Lotus/Types/Items/Research/ChemComponent",
      "name": "Detonite Injector",
      "description": "Detonite injectors are the basis for explosive and incendiary weapons.\n\nLocation: Clan Research (Chem Lab in the Dojo) and Grineer Invasion Rewards",
      "itemCount": 5,
      "imageName": "detonite-injector-64a926ce31.png",
      "tradable": false,
      "drops": [],
      "masterable": false
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/Forma",
      "name": "Forma",
      "description": "This shape-altering component is fundamental to Orokin construction. Allows you to change the polarity of a mod slot on a Warframe, Companions or Weapon and then resets their affinity to Unranked. This can only be used on max Rank Warframes, Companions and Weapons.",
      "itemCount": 1,
      "imageName": "forma-778e068bc2.png",
      "tradable": false,
      "drops": [],
      "masterable": false
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/Salvage",
      "name": "Salvage",
      "description": "High value metals collected from war salvage.\n\nLocation: Mars, Jupiter, and Sedna",
      "itemCount": 7500,
      "imageName": "salvage-b2174bedac.png",
      "tradable": false,
      "drops": [],
      "masterable": false,
      "type": "Resource"
    }
  ],
  "consumeOnBuild": true,
  "criticalChance": 0.2,
  "criticalMultiplier": 2,
  "damage": {
    "total": 149,
    "impact": 14.900001,
    "puncture": 119.2,
    "slash": 14.900001,
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
    14.900001,
    14.900001,
    119.2,
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
  "description": "Tyl Regor’s custom axe and shield are how he likes to eliminate ‘frustrations’.",
  "disposition": 5,
  "fireRate": 0.83333337,
  "followThrough": 0.60000002,
  "heavyAttackDamage": 745,
  "heavySlamAttack": 596,
  "heavySlamRadialDamage": 447,
  "heavySlamRadius": 8,
  "imageName": "ack-&-brunt-2e80c10c0d.png",
  "introduced": {
    "name": "Update 17.0",
    "url": "https://wiki.warframe.com/w/Update_17%23Update_17.0",
    "aliases": [
      "Update 17",
      "17.0",
      "17"
    ],
    "parent": "17.0",
    "date": "2015-07-31"
  },
  "isPrime": false,
  "marketCost": 150,
  "masterable": true,
  "masteryReq": 3,
  "name": "Ack & Brunt",
  "omegaAttenuation": 1.35,
  "patchlogs": [
    {
      "name": "Update 29.5: Deimos Arcana",
      "date": "2020-11-19T23:39:27Z",
      "url": "https://forums.warframe.com/topic/1236257-update-295-deimos-arcana/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Ack & Brunt not fully unfolding when previewing it in the Arsenal."
    },
    {
      "name": "Oberon Prime: Hotfix 20.6.2",
      "date": "2017-05-30T18:11:50Z",
      "url": "https://forums.warframe.com/topic/801884-oberon-prime-hotfix-2062/",
      "additions": "",
      "changes": "Silva & Aegis no longer benefits from blocking Elemental attacks, as it was intended to be a function for the Ack & Brunt. \nAck & Brunt blocking now gains Elemental damage based on the Elemental damage type it blocked.",
      "fixes": ""
    },
    {
      "name": "Hotfix 18.10.4 Operation Rathuum.",
      "date": "2016-05-02T21:29:22Z",
      "url": "https://forums.warframe.com/topic/644355-hotfix-18104-operation-rathuum/",
      "additions": "Added a Range description to the Ack & Brunt Electromagnetic Shielding Mod.",
      "changes": "",
      "fixes": ""
    },
    {
      "name": "Update 17.9.0",
      "date": "2015-10-28T15:12:27Z",
      "url": "https://forums.warframe.com/topic/552544-update-1790/",
      "additions": "Ack & Brunt",
      "changes": "",
      "fixes": ""
    },
    {
      "name": "Hotfix 17.5.4",
      "date": "2015-10-02T22:13:17Z",
      "url": "https://forums.warframe.com/topic/537213-hotfix-1754/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Ack & Brunt slide attack not scaling the model while equipped."
    },
    {
      "name": "Update 17.1.0 - 17.1.4",
      "date": "2015-08-12T21:08:45Z",
      "url": "https://forums.warframe.com/topic/509640-update-1710-1714/",
      "additions": "",
      "changes": "This bundle of Nightwatch Skins suits the Ack & Brunt, Ogris and Nukor. They have been modeled after the camouflage employed by the Nightwatch Corps, an elite squadron in the Grineer Military.",
      "fixes": ""
    },
    {
      "name": "Hotfix 17.0.3",
      "date": "2015-08-04T22:21:45Z",
      "url": "https://forums.warframe.com/topic/503583-hotfix-1703/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Twin Grakatas and Ack & Brunt not properly using player’s custom Energy color."
    },
    {
      "name": "Update 17: Echoes Of The Sentient",
      "date": "2015-07-31T15:09:25Z",
      "url": "https://forums.warframe.com/topic/498793-update-17-echoes-of-the-sentient/",
      "additions": "",
      "changes": "Ack & Brunt ",
      "fixes": ""
    }
  ],
  "procChance": 0.10000002,
  "productCategory": "Melee",
  "range": 2.5,
  "releaseDate": "2015-07-31",
  "skipBuildTimePrice": 35,
  "slamAttack": 447,
  "slamRadialDamage": 298,
  "slamRadius": 7,
  "slideAttack": 149,
  "slot": 5,
  "stancePolarity": "madurai",
  "tags": [
    "Grineer"
  ],
  "totalDamage": 149,
  "tradable": false,
  "type": "Melee",
  "uniqueName": "/Lotus/Weapons/Grineer/Melee/GrineerTylAxeAndBoar/RegorAxeShield",
  "wikiAvailable": true,
  "wikiaUrl": "https://wiki.warframe.com/w/Ack_%26_Brunt",
  "windUp": 0.69999999
}
"#;

        let rec: Melee = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/Grineer/Melee/GrineerTylAxeAndBoar/RegorAxeShield"
        );
    }
}

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<Secondary>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Secondary {
    pub accuracy: f64,
    #[serde(default)]
    pub attacks: Vec<Attack>,
    pub bp_cost: Option<i64>,
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
    pub disposition: Option<i64>,
    pub fire_rate: f64,
    pub image_name: String,
    pub introduced: Option<Introduced2>,
    pub is_prime: bool,
    pub magazine_size: Option<i64>,
    pub masterable: bool,
    pub mastery_req: i64,
    pub multishot: i64,
    pub name: String,
    pub noise: String,
    pub omega_attenuation: f64,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    pub proc_chance: f64,
    pub product_category: String,
    pub release_date: Option<String>,
    pub reload_time: f64,
    pub skip_build_time_price: Option<i64>,
    pub slot: i64,
    #[serde(default)]
    pub tags: Vec<String>,
    pub total_damage: f64,
    pub tradable: bool,
    pub trigger: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub wiki_available: Option<bool>,
    pub wikia_thumbnail: Option<String>,
    pub wikia_url: Option<String>,
    pub market_cost: Option<i64>,
    #[serde(default)]
    pub polarities: Vec<String>,
    pub estimated_vault_date: Option<String>,
    pub vaulted: Option<bool>,
    pub item_count: Option<i64>,
    #[serde(default)]
    pub parents: Vec<String>,
    pub vault_date: Option<String>,
    #[serde(default)]
    pub drops: Vec<Drop2>,
    pub max_level_cap: Option<i64>,
}

impl ProductCategory for Secondary {
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
    pub damage: Damage,
    pub falloff: Option<Falloff>,
    #[serde(rename = "shot_speed")]
    pub shot_speed: Option<i64>,
    pub flight: Option<i64>,
    #[serde(rename = "charge_time")]
    pub charge_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub impact: Option<f64>,
    pub slash: Option<f64>,
    pub puncture: Option<f64>,
    pub heat: Option<i64>,
    pub radiation: Option<i64>,
    pub toxin: Option<i64>,
    pub blast: Option<i64>,
    pub electricity: Option<i64>,
    pub corrosive: Option<i64>,
    pub gas: Option<i64>,
    pub magnetic: Option<i64>,
    pub cold: Option<i64>,
    pub viral: Option<i64>,
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
    pub prime_selling_price: Option<i64>,
    pub ducats: Option<i64>,
    pub vaulted: Option<bool>,
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
    pub estimated_vault_date: Option<String>,
    pub vault_date: Option<String>,
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
#[serde(rename_all = "camelCase")]
pub struct Attack2 {
    pub name: String,
    pub speed: f64,
    #[serde(rename = "crit_chance")]
    pub crit_chance: i64,
    #[serde(rename = "crit_mult")]
    pub crit_mult: f64,
    #[serde(rename = "status_chance")]
    pub status_chance: f64,
    #[serde(rename = "shot_type")]
    pub shot_type: Option<String>,
    pub damage: Damage3,
    #[serde(rename = "shot_speed")]
    pub shot_speed: Option<i64>,
    pub flight: Option<i64>,
    pub slide: Option<String>,
    pub falloff: Option<Falloff2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage3 {
    pub impact: Option<f64>,
    pub slash: Option<f64>,
    pub puncture: Option<f64>,
    pub blast: Option<i64>,
    pub heat: Option<i64>,
    pub radiation: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Falloff2 {
    pub start: i64,
    pub end: i64,
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
  "accuracy": 100,
  "attacks": [
    {
      "name": "Normal Attack",
      "speed": 5,
      "crit_chance": 5,
      "crit_mult": 2,
      "status_chance": 10,
      "shot_type": "Projectile",
      "shot_speed": 65,
      "flight": 65,
      "damage": {
        "toxin": 43
      }
    }
  ],
  "bpCost": 15000,
  "buildPrice": 30000,
  "buildQuantity": 1,
  "buildTime": 86400,
  "category": "Secondary",
  "components": [
    {
      "uniqueName": "/Lotus/Weapons/ClanTech/Bio/AcidDartPistolBlueprint",
      "name": "Blueprint",
      "description": "The Acrid fires an acidic-infused needle.",
      "itemCount": 1,
      "imageName": "blueprint.png",
      "tradable": false,
      "masterable": false,
      "drops": []
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
      "uniqueName": "/Lotus/Types/Items/Research/BioComponent",
      "name": "Mutagen Mass",
      "description": "This living mass can produce weaponized toxins for weaponry.\n\nLocation: Clan Research (Bio Lab in the Dojo) and Infested Outbreak Invasion Rewards",
      "itemCount": 5,
      "imageName": "mutagen-mass-17e29a3c73.png",
      "tradable": false,
      "drops": [],
      "masterable": false
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/Nanospores",
      "name": "Nano Spores",
      "description": "Fibrous technocyte tumour. Handle Infested tissue with caution.\n\nLocation: Saturn, Neptune, Eris, and Deimos",
      "itemCount": 5000,
      "imageName": "nano-spores-8933019524.png",
      "tradable": false,
      "drops": [],
      "masterable": false,
      "type": "Resource"
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/Salvage",
      "name": "Salvage",
      "description": "High value metals collected from war salvage.\n\nLocation: Mars, Jupiter, and Sedna",
      "itemCount": 6000,
      "imageName": "salvage-b2174bedac.png",
      "tradable": false,
      "drops": [],
      "masterable": false,
      "type": "Resource"
    }
  ],
  "consumeOnBuild": true,
  "criticalChance": 0.050000001,
  "criticalMultiplier": 2,
  "damage": {
    "total": 43,
    "impact": 0,
    "puncture": 0,
    "slash": 0,
    "heat": 0,
    "cold": 0,
    "electricity": 0,
    "toxin": 43,
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
    0,
    0,
    0,
    0,
    0,
    0,
    43,
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
  "description": "The Acrid fires an acidic-infused needle.",
  "disposition": 5,
  "fireRate": 5.0000005,
  "imageName": "acrid-6363c31728.png",
  "introduced": {
    "name": "Update 8.0",
    "url": "https://wiki.warframe.com/w/Update_8%23Update_8.0",
    "aliases": [
      "8",
      "8.0",
      "Update 8"
    ],
    "parent": "8.0",
    "date": "2013-05-23"
  },
  "isPrime": false,
  "magazineSize": 15,
  "masterable": true,
  "masteryReq": 7,
  "multishot": 1,
  "name": "Acrid",
  "noise": "Alarming",
  "omegaAttenuation": 1.33,
  "patchlogs": [
    {
      "name": "Empyrean: Hotfix 27.0.1",
      "date": "2019-12-13T06:07:49Z",
      "url": "https://forums.warframe.com/topic/1151534-empyrean-hotfix-2701/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed a crash when firing the Acrid."
    },
    {
      "name": "The Jovian Concord: Update 25.1.0",
      "date": "2019-06-05T18:36:15Z",
      "url": "https://forums.warframe.com/topic/1100452-the-jovian-concord-update-2510/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Acrid Energy color persisting as default."
    },
    {
      "name": "Shrine of the Eidolon: Update 22.17.0 + 22.17.0.1",
      "date": "2018-03-28T18:49:46Z",
      "url": "https://forums.warframe.com/topic/939097-shrine-of-the-eidolon-update-22170-221701/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed cases of the Teralyst literally dissolving when killed with the Acrid."
    },
    {
      "name": "The War Within: Update 19.4.2 + 19.4.2.1",
      "date": "2016-12-20T23:37:56Z",
      "url": "https://forums.warframe.com/topic/737306-the-war-within-update-1942-19421/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Acrid’s dissolve ending too early, preventing corpses from being destroyed."
    },
    {
      "name": "The War Within: Update 19.2.0",
      "date": "2016-12-01T21:22:08Z",
      "url": "https://forums.warframe.com/topic/728401-the-war-within-update-1920/",
      "additions": "",
      "changes": "",
      "fixes": "Fix an issue where the Acrid's dissolve would stop prematurely, preventing the corpses from being destroyed. "
    },
    {
      "name": "Update 18: The Second Dream",
      "date": "2015-12-03T23:46:40Z",
      "url": "https://forums.warframe.com/topic/568455-update-18-the-second-dream/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed the Acrid’s Toxin DoT not applying to Osprey enemies."
    },
    {
      "name": "Warframe: Archwing",
      "date": "2014-10-24T12:52:08Z",
      "url": "https://forums.warframe.com/topic/328727-warframe-archwing/",
      "additions": "",
      "changes": "Acrid: Toxic Sequence: adds 50% status duration.        ",
      "fixes": ""
    },
    {
      "name": "Update 14.2.0: Avalanche Offensive",
      "date": "2014-08-13T22:39:01Z",
      "url": "https://forums.warframe.com/topic/289522-update-1420-avalanche-offensive/",
      "additions": "",
      "changes": "",
      "fixes": "Update 14.2.0: Avalanche Offensive Psa: The Launcher May Require You To Enter Your Windows Administrator Password To Update Your Firewall Rules.Avalanche Offensive Is On Until Monday @ 12 Noon Edt! Avalanche Offensivedefeat The Corpus Arctic Eximus!A Message From Cantis Of The Red Veil Resistance:Here'S What You'Ll Need To Dogo Forth And Take Down The Corpus Arctic Eximus, Tenno!Additions:Weapon Balance Changes:     • Grakata    • Castanas    • Ogris    • Torid    • Penta    • Lanka    • Vectis    • Vulkar    • Angstrum:    • Acrid:    • Attica:    • Spectra & Flux Rifle:    • Snipetron (+Vandal):    • Torid Changes:    • All Launchers:Key Hosting Improvements:Changes:Fixes:"
    },
    {
      "name": "Hotfix 14.0.11",
      "date": "2014-07-29T22:08:19Z",
      "url": "https://forums.warframe.com/topic/278042-hotfix-14011/",
      "additions": "",
      "changes": "",
      "fixes": "Acrid damage multiplier from 1 to .25"
    },
    {
      "name": "Hotfix 11.0.1",
      "date": "2013-11-21T05:54:39Z",
      "url": "https://forums.warframe.com/topic/133329-hotfix-1101/",
      "additions": "",
      "changes": "Acrid Pistol, damage increases.",
      "fixes": ""
    },
    {
      "name": "Update 11: Valkyr Unleashed",
      "date": "2013-11-21T00:23:32Z",
      "url": "https://forums.warframe.com/topic/132825-update-11-valkyr-unleashed/",
      "additions": "",
      "changes": "Fixed Acrid weapon not doing damage over time",
      "fixes": ""
    },
    {
      "name": "Update 10.5.0: The Gradivus Dilemma",
      "date": "2013-10-24T03:55:25Z",
      "url": "https://forums.warframe.com/topic/122841-update-1050-the-gradivus-dilemma/",
      "additions": "",
      "changes": "Corrupted “Magnum Force” mod now works with Acrid.",
      "fixes": ""
    },
    {
      "name": "Update 9.5: Operation Arid Fear",
      "date": "2013-08-09T21:23:06Z",
      "url": "https://forums.warframe.com/topic/92568-update-95-operation-arid-fear/",
      "additions": "",
      "changes": "Added custom reload sounds for Strun, Dera, Supra, Gorgon, Ignis, Grakata, Acrid, Kraken, Hek, and Lex.",
      "fixes": ""
    },
    {
      "name": "Update 9: Vor's Revenge",
      "date": "2013-07-13T01:52:41Z",
      "url": "https://forums.warframe.com/topic/77575-update-9-vors-revenge/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Acrid DOT numbers not showing up on capture target, and death affect not replicating."
    },
    {
      "name": "Hotfix 8.2: Tenno Reinforcement",
      "date": "2013-06-28T17:08:23Z",
      "url": "https://forums.warframe.com/topic/70880-hotfix-82-tenno-reinforcement/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Acrid’s DOT not always working on client."
    },
    {
      "name": "Hotfix 8.1.3",
      "date": "2013-06-19T13:43:51Z",
      "url": "https://forums.warframe.com/topic/66795-hotfix-813/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Hornet Strike not affecting DOT from the Acrid"
    },
    {
      "name": "Hotfix 8.1.2",
      "date": "2013-06-13T14:02:52Z",
      "url": "https://forums.warframe.com/topic/64338-hotfix-812/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Acrid darts not disappearing"
    }
  ],
  "procChance": 0.10000002,
  "productCategory": "Pistols",
  "releaseDate": "2013-05-23",
  "reloadTime": 1.2,
  "skipBuildTimePrice": 35,
  "slot": 0,
  "tags": [
    "Grineer"
  ],
  "totalDamage": 43,
  "tradable": false,
  "trigger": "Semi",
  "type": "Pistol",
  "uniqueName": "/Lotus/Weapons/ClanTech/Bio/AcidDartPistol",
  "wikiAvailable": true,
  "wikiaThumbnail": "https://wiki.warframe.com/images/Acrid.png?57294",
  "wikiaUrl": "https://wiki.warframe.com/w/Acrid"
}
"#;

        let rec: Secondary = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/ClanTech/Bio/AcidDartPistol"
        );
    }
}

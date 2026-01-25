use serde::{Deserialize, Serialize};

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
        let json_data = r#"
{
  "accuracy": 12.5,
  "attacks": [
    {
      "name": "1st Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "Projectile",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "blast": 16
      }
    },
    {
      "name": "1st Attack Radial Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "AoE",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "blast": 114
      }
    },
    {
      "name": "2nd Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "Projectile",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "corrosive": 16
      }
    },
    {
      "name": "2nd Attack Radial Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "AoE",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "corrosive": 114
      }
    },
    {
      "name": "3rd Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "Projectile",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "gas": 16
      }
    },
    {
      "name": "3rd Attack Radial Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "AoE",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "gas": 114
      }
    },
    {
      "name": "4th Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "Projectile",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "magnetic": 16
      }
    },
    {
      "name": "4th Attack Radial Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "AoE",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "magnetic": 114
      }
    },
    {
      "name": "5th Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "Projectile",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "radiation": 16
      }
    },
    {
      "name": "5th Attack Radial Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "AoE",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "radiation": 114
      }
    },
    {
      "name": "6th Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "Projectile",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "viral": 16
      }
    },
    {
      "name": "6th Attack Radial Attack",
      "speed": 1.5,
      "crit_chance": 10,
      "crit_mult": 2.9,
      "status_chance": 34.9,
      "shot_type": "AoE",
      "falloff": {
        "start": 1500,
        "end": 2003,
        "reduction": 1
      },
      "damage": {
        "viral": 114
      }
    }
  ],
  "buildPrice": 25000,
  "buildQuantity": 1,
  "buildTime": 43200,
  "category": "Arch-Gun",
  "components": [
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/WeaponParts/NokkoArchGunBarrelItem",
      "name": "Barrel",
      "description": "A weapon-crafting component.",
      "excludeFromCodex": true,
      "itemCount": 1,
      "imageName": "barrel.png",
      "tradable": false,
      "drops": [
        {
          "chance": 0.025,
          "location": "Nokko: Corporate Restructuring Rewards, Rotation B",
          "rarity": "Rare",
          "type": "Arbucep Barrel Blueprint"
        },
        {
          "chance": 0.05,
          "location": "Nokko: Corporate Restructuring (Steel Path), Rotation B",
          "rarity": "Rare",
          "type": "Arbucep Barrel Blueprint"
        },
        {
          "chance": 0.1,
          "location": "Nokko: Corporate Restructuring (Steel Path), Rotation C",
          "rarity": "Uncommon",
          "type": "Arbucep Barrel Blueprint"
        },
        {
          "chance": 0.1,
          "location": "Nokko: Corporate Restructuring Rewards, Rotation C",
          "rarity": "Uncommon",
          "type": "Arbucep Barrel Blueprint"
        }
      ],
      "masterable": false
    },
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/NokkoArchGunBlueprint",
      "name": "Blueprint",
      "description": "Nokko’s signature archgun fires six homing missiles, each bearing a payload of one of the six combined elements which detonate in an area upon impact.",
      "itemCount": 1,
      "imageName": "blueprint.png",
      "tradable": false,
      "masterable": false,
      "drops": [
        {
          "chance": 0.05,
          "location": "Nokko: Corporate Restructuring Rewards, Rotation C",
          "rarity": "Rare",
          "type": "Arbucep Blueprint"
        },
        {
          "chance": 0.05,
          "location": "Nokko: Critter Liberation Rewards, Rotation C",
          "rarity": "Rare",
          "type": "Arbucep Blueprint"
        },
        {
          "chance": 0.05,
          "location": "Nokko: Weed The Gardens Rewards, Rotation C",
          "rarity": "Rare",
          "type": "Arbucep Blueprint"
        },
        {
          "chance": 0.08,
          "location": "Nokko: Corporate Restructuring (Steel Path), Rotation C",
          "rarity": "Rare",
          "type": "Arbucep Blueprint"
        },
        {
          "chance": 0.08,
          "location": "Nokko: Critter Liberation Rewards (Steel Path), Rotation C",
          "rarity": "Rare",
          "type": "Arbucep Blueprint"
        },
        {
          "chance": 0.08,
          "location": "Nokko: Weed The Gardens Rewards (Steel Path), Rotation C",
          "rarity": "Rare",
          "type": "Arbucep Blueprint"
        }
      ]
    },
    {
      "uniqueName": "/Lotus/Types/Items/MushroomJournal/PlainMushroomJournalItem",
      "name": "Dull Button",
      "description": "This mushroom is as plain as plain can be. Take it to Nightcap in the airlock for further information.\n\nLocation: Deepmines Bounties on Venus",
      "excludeFromCodex": true,
      "itemCount": 5,
      "imageName": "dull-button-c660fcc44a.png",
      "tradable": false,
      "masterable": false,
      "drops": []
    },
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/WeaponParts/NokkoArchGunReceiverItem",
      "name": "Receiver",
      "description": "A weapon-crafting component.",
      "excludeFromCodex": true,
      "itemCount": 1,
      "imageName": "receiver.png",
      "tradable": false,
      "drops": [
        {
          "chance": 0.025,
          "location": "Nokko: Critter Liberation Rewards, Rotation B",
          "rarity": "Rare",
          "type": "Arbucep Receiver Blueprint"
        },
        {
          "chance": 0.05,
          "location": "Nokko: Critter Liberation Rewards (Steel Path), Rotation B",
          "rarity": "Rare",
          "type": "Arbucep Receiver Blueprint"
        },
        {
          "chance": 0.1,
          "location": "Nokko: Critter Liberation Rewards (Steel Path), Rotation C",
          "rarity": "Uncommon",
          "type": "Arbucep Receiver Blueprint"
        },
        {
          "chance": 0.1,
          "location": "Nokko: Critter Liberation Rewards, Rotation C",
          "rarity": "Uncommon",
          "type": "Arbucep Receiver Blueprint"
        }
      ],
      "masterable": false
    },
    {
      "uniqueName": "/Lotus/Types/Recipes/Weapons/WeaponParts/NokkoArchGunStockItem",
      "name": "Stock",
      "description": "A weapon-crafting component.",
      "excludeFromCodex": true,
      "itemCount": 1,
      "imageName": "stock.png",
      "tradable": false,
      "drops": [
        {
          "chance": 0.025,
          "location": "Nokko: Weed The Gardens Rewards, Rotation B",
          "rarity": "Rare",
          "type": "Arbucep Stock Blueprint"
        },
        {
          "chance": 0.05,
          "location": "Nokko: Weed The Gardens Rewards (Steel Path), Rotation B",
          "rarity": "Rare",
          "type": "Arbucep Stock Blueprint"
        },
        {
          "chance": 0.1,
          "location": "Nokko: Weed The Gardens Rewards (Steel Path), Rotation C",
          "rarity": "Uncommon",
          "type": "Arbucep Stock Blueprint"
        },
        {
          "chance": 0.1,
          "location": "Nokko: Weed The Gardens Rewards, Rotation C",
          "rarity": "Uncommon",
          "type": "Arbucep Stock Blueprint"
        }
      ],
      "masterable": false
    }
  ],
  "consumeOnBuild": true,
  "criticalChance": 0.1,
  "criticalMultiplier": 2.9000001,
  "damage": {
    "total": 130,
    "impact": 0,
    "puncture": 0,
    "slash": 0,
    "heat": 0,
    "cold": 0,
    "electricity": 0,
    "toxin": 0,
    "blast": 130,
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
    0,
    130,
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
  "description": "Nokko’s signature archgun fires six homing missiles, each bearing a payload of one of the six combined elements which detonate in an area upon impact.",
  "disposition": 1,
  "fireRate": 1.5000001,
  "imageName": "arbucep-6cabdc8f25.png",
  "introduced": {
    "name": "Update 40.0",
    "url": "https://wiki.warframe.com/w/Update_40%23Update_40.0",
    "aliases": [
      "40",
      "40.0"
    ],
    "parent": "40.0",
    "date": "2025-10-15"
  },
  "isPrime": false,
  "magazineSize": 6,
  "marketCost": 240,
  "masterable": true,
  "masteryReq": 0,
  "multishot": 6,
  "name": "Arbucep",
  "noise": "Alarming",
  "omegaAttenuation": 0.60000002,
  "polarities": [
    "madurai",
    "naramon"
  ],
  "procChance": 0.34900004,
  "productCategory": "SpaceGuns",
  "releaseDate": "2025-10-15",
  "reloadTime": 1.72,
  "skipBuildTimePrice": 50,
  "slot": 1,
  "tags": [
    "Tenno"
  ],
  "totalDamage": 130,
  "tradable": false,
  "trigger": "Auto",
  "type": "Arch-Gun",
  "uniqueName": "/Lotus/Weapons/Tenno/Archwing/Primary/NokkoArchGun/NokkoArchGun",
  "wikiAvailable": true,
  "wikiaThumbnail": "https://wiki.warframe.com/images/Arbucep.png?37aa2",
  "wikiaUrl": "https://wiki.warframe.com/w/Arbucep"
}
"#;

        let rec: ArchGun = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Weapons/Tenno/Archwing/Primary/NokkoArchGun/NokkoArchGun"
        );
    }
}

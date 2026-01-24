use serde::{Deserialize, Serialize};

use crate::itemdata::{DropChance, Named, Noise, PatchLog, Rarity, Trigger};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchGun {
    #[serde(flatten)]
    pub named: Named,

    #[serde(rename = "type")]
    pub type_: String,

    pub description: Option<String>,

    pub noise: Noise,
    pub trigger: Trigger,
    pub disposition: Option<u8>,

    pub rarity: Option<Rarity>,
    pub drops: Option<Vec<DropChance>>,

    #[serde(rename = "imageName")]
    pub image_name: Option<String>,

    #[serde(rename = "masteryReq")]
    pub mastery_req: Option<u8>,

    #[serde(rename = "tradable")]
    pub tradable: Option<bool>,

    #[serde(rename = "masterable")]
    pub masterable: Option<bool>,

    #[serde(rename = "isPrime")]
    pub is_prime: Option<bool>,
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
            rec.named.unique_name,
            "/Lotus/Weapons/Tenno/Archwing/Primary/NokkoArchGun/NokkoArchGun"
        );
    }
}

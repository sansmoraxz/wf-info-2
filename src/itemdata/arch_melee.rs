use multi_index_map::MultiIndexMap;
use serde::{Deserialize, Serialize};

use crate::itemdata::{DropChance, PatchLog, Rarity};

#[derive(Debug, MultiIndexMap, Serialize, Deserialize)]
#[multi_index_derive(Debug)]
#[multi_index_hash(rustc_hash::FxBuildHasher)]
pub struct ArchMelee {
    #[multi_index(hashed_non_unique)]
    #[serde(rename = "name")]
    pub name: String,

    #[multi_index(hashed_unique)]
    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[multi_index(hashed_non_unique)]
    #[serde(rename = "type")]
    pub type_: String,

    pub description: Option<String>,

    pub range: Option<f32>,
    pub disposition: Option<u8>,

    pub rarity: Option<Rarity>,
    pub drops: Option<Vec<DropChance>>,

    #[serde(rename = "imageName")]
    pub image_name: Option<String>,

    #[serde(rename = "masteryReq")]
    pub mastery_req: Option<u8>,

    #[serde(rename = "patchlogs")]
    pub patch_log: Option<Vec<PatchLog>>,

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

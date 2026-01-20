use serde::{Deserialize, Serialize};

use crate::itemdata::{DropChance, LevelStats, PatchLog, Rarity};

#[derive(Debug, Serialize, Deserialize)]
pub struct Arcane {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[serde(rename = "type")]
    pub r#type: String,

    pub rarity: Option<Rarity>,
    pub drops: Option<Vec<DropChance>>,

    #[serde(rename = "imageName")]
    pub image_name: String,

    #[serde(rename = "levelStats")]
    pub level_stats: Option<Vec<LevelStats>>,

    #[serde(rename = "patchlogs")]
    pub patch_log: Option<PatchLog>,

    #[serde(rename = "tradable")]
    pub tradable: bool,

    #[serde(rename = "masterable")]
    pub masterable: bool,

    #[serde(rename = "excludeFromCodex")]
    pub exclude_from_codex: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_long_gun() {
        let json_data = r#"
{
  "category": "Arcanes",
  "drops": [
    {
      "chance": 0.0443,
      "location": "Duviri/Endless: Tier 1 (Hard)",
      "rarity": "Rare",
      "type": "Akimbo Slip Shot"
    },
    {
      "chance": 0.0443,
      "location": "Duviri/Endless: Tier 3 (Hard)",
      "rarity": "Rare",
      "type": "Akimbo Slip Shot"
    },
    {
      "chance": 0.0443,
      "location": "Duviri/Endless: Tier 4 (Hard)",
      "rarity": "Rare",
      "type": "Akimbo Slip Shot"
    },
    {
      "chance": 0.06,
      "location": "Duviri/Endless: Tier 6 (Hard)",
      "rarity": "Rare",
      "type": "Akimbo Slip Shot"
    },
    {
      "chance": 0.06,
      "location": "Duviri/Endless: Tier 7 (Hard)",
      "rarity": "Rare",
      "type": "Akimbo Slip Shot"
    },
    {
      "chance": 0.0909,
      "location": "Duviri/Endless: Repeated Rewards (Hard)",
      "rarity": "Rare",
      "type": "Akimbo Slip Shot"
    },
    {
      "chance": 0.1019,
      "location": "Duviri Static Undercroft Portal (Steel Path)",
      "rarity": "Uncommon",
      "type": "Akimbo Slip Shot"
    }
  ],
  "imageName": "akimbo-slip-shot-2b668cc242.png",
  "levelStats": [
    {
      "stats": [
        "While sliding or aim gliding: Gain 15% ammo efficiency with Dual Pistols."
      ]
    },
    {
      "stats": [
        "While sliding or aim gliding: Gain 25% ammo efficiency with Dual Pistols."
      ]
    },
    {
      "stats": [
        "While sliding or aim gliding: Gain 35% ammo efficiency with Dual Pistols."
      ]
    },
    {
      "stats": [
        "While sliding or aim gliding: Gain 45% ammo efficiency with Dual Pistols."
      ]
    },
    {
      "stats": [
        "While sliding or aim gliding: Gain 55% ammo efficiency with Dual Pistols."
      ]
    },
    {
      "stats": [
        "While sliding or aim gliding: Gain 65% ammo efficiency with Dual Pistols."
      ]
    }
  ],
  "masterable": false,
  "name": "Akimbo Slip Shot",
  "rarity": "Rare",
  "tradable": true,
  "type": "Secondary Arcane",
  "uniqueName": "/Lotus/Upgrades/CosmeticEnhancers/Offensive/AmmoEfficiencyOnSliding"
}
"#;

        let rec: Arcane = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Upgrades/CosmeticEnhancers/Offensive/AmmoEfficiencyOnSliding"
        );
    }
}

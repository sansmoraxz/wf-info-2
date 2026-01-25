use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<Arcane>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arcane {
    pub category: String,
    #[serde(default)]
    pub drops: Vec<Drop>,
    pub image_name: String,
    #[serde(default)]
    pub level_stats: Vec<LevelStat>,
    pub masterable: bool,
    pub name: String,
    pub rarity: Option<String>,
    pub tradable: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub exclude_from_codex: Option<bool>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub components: Option<Vec<Component>>,
    pub consume_on_build: Option<bool>,
    pub skip_build_time_price: Option<i64>,
}

impl ProductCategory for Arcane {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Upgrades".to_string(), "RawUpgrades".to_string()]
    }
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
pub struct LevelStat {
    pub stats: Vec<String>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub unique_name: String,
    pub name: String,
    pub item_count: i64,
    pub image_name: String,
    pub tradable: bool,
    pub masterable: bool,
    pub drops: Vec<Drop>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_arcane() {
        let json_data = r#"
{
  "category": "Arcanes",
  "drops": [
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 1 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.0067,
      "location": "Duviri/Endless: Tier 3 (Normal)",
      "rarity": "Legendary",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.037,
      "location": "The Descendia: Infernum 20 Triple Arcane Rewards",
      "rarity": "Rare",
      "type": "3X Arcane Agility"
    },
    {
      "chance": 0.04,
      "location": "Duviri/Endless: Tier 9 (Normal)",
      "rarity": "Rare",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.0563,
      "location": "Veil/Erato (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.0667,
      "location": "Pluto/Khufu Envoy (Skirmish), Rotation C",
      "rarity": "Rare",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.0714,
      "location": "Duviri Static Undercroft Portal",
      "rarity": "Rare",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.0925,
      "location": "Eidolon Gantulyst (Capture)",
      "rarity": "Rare",
      "type": "Arcane Agility"
    },
    {
      "chance": 0.102,
      "location": "Eidolon Gantulyst",
      "rarity": "Uncommon",
      "type": "Arcane Agility"
    }
  ],
  "imageName": "arcane-agility-ad41572e0e.png",
  "levelStats": [
    {
      "stats": [
        "On Damaged:\\n60% chance for +10% Parkour Velocity for 18s"
      ]
    },
    {
      "stats": [
        "On Damaged:\\n60% chance for +20% Parkour Velocity for 18s"
      ]
    },
    {
      "stats": [
        "On Damaged:\\n60% chance for +30% Parkour Velocity for 18s"
      ]
    },
    {
      "stats": [
        "On Damaged:\\n60% chance for +40% Parkour Velocity for 18s\\n+1 Arcane Revive",
        "+1 Arcane Revive"
      ]
    },
    {
      "stats": [
        "On Damaged:\\n60% chance for +50% Parkour Velocity for 18s\\n+1 Arcane Revive",
        "+1 Arcane Revive"
      ]
    },
    {
      "stats": [
        "On Damaged:\\n60% chance for +60% Parkour Velocity for 18s\\n+1 Arcane Revive",
        "+1 Arcane Revive"
      ]
    }
  ],
  "masterable": false,
  "name": "Arcane Agility",
  "patchlogs": [
    {
      "name": "Warframe Revised: Update 27.2.0",
      "date": "2020-03-05T15:32:32Z",
      "url": "https://forums.warframe.com/topic/1172454-warframe-revised-update-2720/",
      "additions": "",
      "changes": "Arcane Agility:",
      "fixes": ""
    }
  ],
  "rarity": "Uncommon",
  "tradable": true,
  "type": "Warframe Arcane",
  "uniqueName": "/Lotus/Upgrades/CosmeticEnhancers/Defensive/SpeedOnDamage"
}
"#;

        let rec: Arcane = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Upgrades/CosmeticEnhancers/Defensive/SpeedOnDamage"
        );
    }
}

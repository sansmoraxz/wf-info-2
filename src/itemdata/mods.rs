use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<Mod>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    pub base_drain: Option<i64>,
    pub category: String,
    pub compat_name: Option<String>,
    #[serde(default)]
    pub drops: Vec<Drop>,
    pub fusion_limit: Option<i64>,
    pub image_name: String,
    pub introduced: Option<Introduced>,
    pub is_augment: Option<bool>,
    pub is_prime: bool,
    #[serde(default)]
    pub level_stats: Vec<LevelStat>,
    pub masterable: bool,
    pub name: String,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    pub polarity: Option<String>,
    pub rarity: Option<String>,
    pub release_date: Option<String>,
    pub tradable: bool,
    pub transmutable: Option<bool>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub wiki_available: Option<bool>,
    pub wikia_thumbnail: Option<String>,
    pub wikia_url: Option<String>,
    pub is_utility: Option<bool>,
    pub mod_set: Option<String>,
    pub exclude_from_codex: Option<bool>,
    pub is_exilus: Option<bool>,
    pub description: Option<String>,
    pub num_upgrades_in_set: Option<i64>,
    #[serde(default)]
    pub stats: Vec<String>,
    #[serde(default)]
    pub available_challenges: Vec<AvailableChallenge>,
    #[serde(default)]
    pub upgrade_entries: Vec<UpgradeEntry>,
    pub buff_set: Option<bool>,
    pub mod_set_values: Option<Vec<f64>>,
}

impl ProductCategory for Mod {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Upgrades".to_string(), "RawUpgrades".to_string()]
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drop {
    pub chance: Option<f64>,
    pub location: String,
    pub rarity: String,
    #[serde(rename = "type")]
    pub type_field: String,
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
pub struct LevelStat {
    pub stats: Vec<String>,
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
pub struct AvailableChallenge {
    pub full_name: String,
    pub description: String,
    pub complications: Vec<Complication>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Complication {
    pub full_name: String,
    pub description: String,
    pub override_tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeEntry {
    pub tag: String,
    pub prefix_tag: String,
    pub suffix_tag: String,
    pub upgrade_values: Vec<UpgradeValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeValue {
    pub value: f64,
    pub loc_tag: Option<String>,
    pub reverse_value_symbol: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_raw_mod() {
        let json_data = r#"
{
  "baseDrain": 4,
  "category": "Mods",
  "compatName": "WARFRAME",
  "drops": [
    {
      "chance": 0.0909,
      "location": "Earth/Cetus (Level 50 - 70 Cetus Bounty), Rotation C",
      "rarity": "Rare",
      "type": "Amar's Hatred"
    },
    {
      "chance": 0.0909,
      "location": "Venus/Orb Vallis (Level 50 - 70 Orb Vallis Bounty), Rotation C",
      "rarity": "Rare",
      "type": "Amar's Hatred"
    },
    {
      "chance": 0.125,
      "location": "Earth/Cetus (Level 50 - 70 Cetus Bounty), Rotation C",
      "rarity": "Uncommon",
      "type": "Amar's Hatred"
    },
    {
      "chance": 0.125,
      "location": "Venus/Orb Vallis (Level 50 - 70 Orb Vallis Bounty), Rotation C",
      "rarity": "Uncommon",
      "type": "Amar's Hatred"
    },
    {
      "chance": 0.1351,
      "location": "Earth/Cetus (Level 50 - 70 Cetus Bounty), Rotation C",
      "rarity": "Uncommon",
      "type": "Amar's Hatred"
    },
    {
      "chance": 0.1351,
      "location": "Venus/Orb Vallis (Level 50 - 70 Orb Vallis Bounty), Rotation C",
      "rarity": "Uncommon",
      "type": "Amar's Hatred"
    }
  ],
  "fusionLimit": 5,
  "imageName": "amar's-hatred-c1e8cbf38a.jpg",
  "introduced": {
    "name": "Update 31.0",
    "url": "https://wiki.warframe.com/w/Update_31%23Update_31.0",
    "aliases": [
      "31",
      "31.0",
      "The New War"
    ],
    "parent": "31.0",
    "date": "2021-12-15"
  },
  "isAugment": true,
  "isPrime": false,
  "levelStats": [
    {
      "stats": [
        "+4% Armor",
        "+2.5% Ability Strength"
      ]
    },
    {
      "stats": [
        "+8% Armor",
        "+5% Ability Strength"
      ]
    },
    {
      "stats": [
        "+13% Armor",
        "+7.5% Ability Strength"
      ]
    },
    {
      "stats": [
        "+17% Armor",
        "+10% Ability Strength"
      ]
    },
    {
      "stats": [
        "+21% Armor",
        "+12.5% Ability Strength"
      ]
    },
    {
      "stats": [
        "+25% Armor",
        "+15% Ability Strength"
      ]
    }
  ],
  "masterable": false,
  "modSet": "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod",
  "name": "Amar's Hatred",
  "polarity": "vazarin",
  "rarity": "Uncommon",
  "releaseDate": "2021-12-15",
  "tradable": true,
  "transmutable": false,
  "type": "Warframe Mod",
  "uniqueName": "/Lotus/Upgrades/Mods/Sets/Amar/AmarWarframeMod",
  "wikiAvailable": true,
  "wikiaThumbnail": "https://wiki.warframe.com/images/Amar%27sHatredMod.png?92f89",
  "wikiaUrl": "https://wiki.warframe.com/w/Amar's_Hatred"
}
"#;

        let m: Mod = from_str(json_data).unwrap();

        assert_eq!(
            m.unique_name,
            "/Lotus/Upgrades/Mods/Sets/Amar/AmarWarframeMod"
        );
        assert_eq!(
            m.mod_set,
            Some("/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod".into())
        );
    }

    #[test]
    fn test_deserialize_mod_set() {
        let json_data = r#"
{
  "category": "Mods",
  "imageName": "amarsetmod-c1bb91549f.png",
  "isPrime": false,
  "masterable": false,
  "name": "Amarsetmod",
  "numUpgradesInSet": 3,
  "stats": [
    "Teleport to a target within 10m on using a Heavy Attack.",
    "Teleport to a target within 20m on using a Heavy Attack.",
    "Teleport to a target within 30m on using a Heavy Attack."
  ],
  "tradable": false,
  "type": "Mod Set Mod",
  "uniqueName": "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod"
}
"#;

        let m: Mod = from_str(json_data).unwrap();

        assert_eq!(m.unique_name, "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod");
        assert_eq!(m.num_upgrades_in_set, Some(3));
    }
}

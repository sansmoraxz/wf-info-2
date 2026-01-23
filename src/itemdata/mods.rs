use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::itemdata::{DropChance, LevelStats, PatchLog, Rarity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMod {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[serde(rename = "type")]
    pub r#type: String,

    pub polarity: String,
    pub transmutable: Option<bool>,
    #[serde(rename = "isAugment")]
    pub is_augment: Option<bool>,
    #[serde(rename = "fusionLimit")]
    pub fusion_limit: u32,

    #[serde(rename = "modSet")]
    pub mod_set: Option<String>,

    pub rarity: Option<Rarity>,
    pub drops: Option<Vec<DropChance>>,

    #[serde(rename = "imageName")]
    pub image_name: String,

    #[serde(rename = "isPrime")]
    pub is_prime: Option<bool>,

    #[serde(rename = "levelStats")]
    pub level_stats: Option<Vec<LevelStats>>,

    #[serde(rename = "patchlogs")]
    pub patch_log: Option<Vec<PatchLog>>,

    #[serde(rename = "tradable")]
    pub tradable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModSets {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[serde(rename = "type")]
    pub r#type: String,

    pub stats: Vec<String>,

    #[serde(rename = "numUpgradesInSet")]
    pub num_upgrades_in_set: u16,

    #[serde(rename = "tradable")]
    pub tradable: bool,

    #[serde(rename = "isPrime")]
    pub is_prime: Option<bool>,
}

#[derive(Debug, Clone)]
pub enum Mod {
    RawMod(RawMod),
    ModSets(ModSets),
    Unknown(Value),
}

impl serde::Serialize for Mod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize the inner struct as a JSON string (to match server/client stringified form)
        let s = match self {
            Mod::RawMod(raw_mod) => serde_json::to_string(raw_mod),
            Mod::ModSets(mod_sets) => serde_json::to_string(mod_sets),
            Mod::Unknown(value) => serde_json::to_string(value),
        }
        .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for Mod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // First deserialize into a serde_json::Value so we can handle both string and object forms.
        let v = serde_json::Value::deserialize(deserializer).map_err(serde::de::Error::custom)?;

        match v {
            serde_json::Value::Object(_) => {
                // When an object is provided directly, try to deserialize into modset then raw mod
                let v_clone = v.clone();
                if let Ok(m) = serde_json::from_value::<ModSets>(v_clone.clone()) {
                    return Ok(Mod::ModSets(m));
                }
                if let Ok(m) = serde_json::from_value::<RawMod>(v_clone.clone()) {
                    return Ok(Mod::RawMod(m));
                }
                Ok(Mod::Unknown(v_clone))
            }
            _ => Err(serde::de::Error::custom(
                "unexpected type for Mod, expected object",
            )),
        }
    }
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

        let rec: Mod = from_str(json_data).unwrap();

        if let Mod::RawMod(m) = rec {
            assert_eq!(
                m.unique_name,
                "/Lotus/Upgrades/Mods/Sets/Amar/AmarWarframeMod"
            );
            assert_eq!(
                m.mod_set.unwrap(),
                "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod"
            );
        } else {
            panic!("expected Mod variant");
        }
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

        let rec: Mod = from_str(json_data).unwrap();

        if let Mod::ModSets(m) = rec {
            assert_eq!(m.unique_name, "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod");
            assert_eq!(m.num_upgrades_in_set, 3);
        } else {
            panic!("expected Mod variant");
        }
    }
}

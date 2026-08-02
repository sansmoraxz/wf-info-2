//! Common types shared across all item categories.

use serde::{Deserialize, Deserializer, Serialize};

use crate::enums::Rarity;

/// Patch/update log entry - records changes made to an item
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patchlog {
    pub name: String,
    pub date: String,
    pub url: String,
    pub additions: String,
    pub changes: String,
    pub fixes: String,
}

/// Game update information - when an item was introduced
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Introduced {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub parent: String,
    pub date: String,
}

/// Drop location and chance information
///
/// Uses Option<f64> for chance to handle null values, with a custom
/// deserializer for both integer and float values from the JSON source.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drop {
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub chance: Option<f64>,
    pub location: String,
    #[serde(default)]
    pub rarity: Rarity,
    #[serde(rename = "type")]
    pub type_field: String,
    pub rotation: Option<String>,
    pub unique_name: Option<String>,
}

/// Level statistics for mods and arcanes
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelStat {
    #[serde(default)]
    pub stats: Vec<String>,
}

/// Ability information for Warframes and Archwings
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub unique_name: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub image_name: String,
}

/// Custom deserializer for numeric fields that may be i64 or f64 in JSON.
///
/// The warframe-items-data source sometimes uses integers and sometimes
/// floats for the same field across different items. This deserializer
/// handles both cases uniformly.
pub fn deserialize_number_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct NumberVisitor;

    impl<'de> Visitor<'de> for NumberVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number (integer or float)")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v as f64)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v as f64)
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }
    }

    deserializer.deserialize_any(NumberVisitor)
}

/// Custom deserializer for optional numeric fields.
pub fn deserialize_option_number_to_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct OptionNumberVisitor;

    impl<'de> Visitor<'de> for OptionNumberVisitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("null or a number")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_number_to_f64(deserializer).map(Some)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v as f64))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v as f64))
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v))
        }
    }

    deserializer.deserialize_any(OptionNumberVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patchlog_deserialize() {
        let json = r#"{
            "name": "Update 30.5",
            "date": "2021-07-06",
            "url": "https://example.com",
            "additions": "Added feature",
            "changes": "Changed behavior",
            "fixes": "Fixed bug"
        }"#;

        let patchlog: Patchlog = serde_json::from_str(json).unwrap();
        assert_eq!(patchlog.name, "Update 30.5");
    }

    #[test]
    fn test_drop_deserialize_with_float_chance() {
        let json = r#"{
            "chance": 0.25,
            "location": "Cetus Bounty",
            "rarity": "Rare",
            "type": "Bounty Reward"
        }"#;

        let drop: Drop = serde_json::from_str(json).unwrap();
        assert!((drop.chance.unwrap() - 0.25).abs() < f64::EPSILON);
        assert_eq!(drop.rarity, Rarity::Rare);
    }

    #[test]
    fn test_drop_deserialize_with_int_chance() {
        let json = r#"{
            "chance": 1,
            "location": "Mission Complete",
            "rarity": "Common",
            "type": "Mission Reward"
        }"#;

        let drop: Drop = serde_json::from_str(json).unwrap();
        assert!((drop.chance.unwrap() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_drop_deserialize_with_null_chance() {
        let json = r#"{
            "chance": null,
            "location": "Enemy Drop",
            "rarity": "Common",
            "type": "Drop"
        }"#;

        let drop: Drop = serde_json::from_str(json).unwrap();
        assert!(drop.chance.is_none());
    }

    #[test]
    fn test_introduced_deserialize() {
        let json = r#"{
            "name": "Vanilla",
            "url": "https://wiki.example.com",
            "aliases": ["0", "0.0"],
            "parent": "0.0",
            "date": "2012-10-25"
        }"#;

        let introduced: Introduced = serde_json::from_str(json).unwrap();
        assert_eq!(introduced.name, "Vanilla");
        assert_eq!(introduced.aliases.len(), 2);
    }

    #[test]
    fn test_level_stat_deserialize() {
        let json = r#"{"stats": ["+10% Damage", "+20% Damage"]}"#;

        let level_stat: LevelStat = serde_json::from_str(json).unwrap();
        assert_eq!(level_stat.stats.len(), 2);
    }
}

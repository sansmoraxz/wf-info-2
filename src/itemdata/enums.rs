//! Enums for known finite value sets in item data.
//!
//! These replace String fields where the set of valid values is known and finite.
//! Each enum includes an `Unknown` variant with `#[serde(other)]` to gracefully
//! handle new values that may be added to the source data in the future.

use serde::{Deserialize, Serialize};

/// Weapon trigger types - how the weapon fires
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Trigger {
    Active,
    Auto,
    #[serde(rename = "Auto Burst")]
    AutoBurst,
    Burst,
    Charge,
    Duplex,
    Held,
    Melee,
    Semi,
    #[default]
    #[serde(other)]
    Unknown,
}

/// Weapon noise level - affects enemy alertness
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Noise {
    Alarming,
    Silent,
    #[default]
    #[serde(other)]
    Unknown,
}

/// Item/drop rarity tiers
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
    #[default]
    #[serde(other)]
    Unknown,
}

/// Mod polarity types - determines mod capacity cost
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Polarity {
    Aura,
    Madurai,
    Naramon,
    Penjaga,
    Umbra,
    Unairu,
    Universal,
    Vazarin,
    Zenurik,
    Any,
    #[default]
    #[serde(other)]
    Unknown,
}

/// Projectile type for ranged weapons
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Projectile {
    Discharge,
    Hitscan,
    Projectile,
    Thrown,
    #[default]
    #[serde(other)]
    Unknown,
}

/// Riven disposition - affects Riven mod stat ranges (1-5)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Disposition {
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "3")]
    Three,
    #[serde(rename = "4")]
    Four,
    #[serde(rename = "5")]
    Five,
    #[default]
    #[serde(other)]
    Unknown,
}

impl Disposition {
    /// Convert disposition to numeric value (1-5), returns 0 for Unknown
    pub fn as_u8(&self) -> u8 {
        match self {
            Disposition::One => 1,
            Disposition::Two => 2,
            Disposition::Three => 3,
            Disposition::Four => 4,
            Disposition::Five => 5,
            Disposition::Unknown => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_deserialize() {
        let trigger: Trigger = serde_json::from_str(r#""Auto Burst""#).unwrap();
        assert_eq!(trigger, Trigger::AutoBurst);

        let trigger: Trigger = serde_json::from_str(r#""Semi""#).unwrap();
        assert_eq!(trigger, Trigger::Semi);

        // Unknown variant for new values
        let trigger: Trigger = serde_json::from_str(r#""NewTriggerType""#).unwrap();
        assert_eq!(trigger, Trigger::Unknown);
    }

    #[test]
    fn test_polarity_deserialize() {
        let polarity: Polarity = serde_json::from_str(r#""madurai""#).unwrap();
        assert_eq!(polarity, Polarity::Madurai);

        let polarity: Polarity = serde_json::from_str(r#""vazarin""#).unwrap();
        assert_eq!(polarity, Polarity::Vazarin);
    }

    #[test]
    fn test_rarity_deserialize() {
        let rarity: Rarity = serde_json::from_str(r#""Legendary""#).unwrap();
        assert_eq!(rarity, Rarity::Legendary);

        let rarity: Rarity = serde_json::from_str(r#""Common""#).unwrap();
        assert_eq!(rarity, Rarity::Common);
    }

    #[test]
    fn test_disposition_as_u8() {
        assert_eq!(Disposition::Three.as_u8(), 3);
        assert_eq!(Disposition::Unknown.as_u8(), 0);
    }
}

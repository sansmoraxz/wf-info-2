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

/// Vault status for Prime items - represents the lifecycle state.
///
/// This is a computed enum derived from the combination of `is_prime`,
/// `vaulted`, `vault_date`, and `estimated_vault_date` fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum VaultStatus {
    /// Not a Prime item (is_prime = false)
    #[default]
    NotPrime,
    /// Prime item currently available (vaulted = false, no estimated date)
    Active,
    /// Prime item with predicted vault date (vaulted = false, has estimated date)
    EstimatedVault { estimated_date: String },
    /// Prime item currently vaulted (vaulted = true)
    Vaulted { date: String },
}

impl VaultStatus {
    /// Compute vault status from individual fields.
    ///
    /// This is the canonical way to derive VaultStatus from the raw JSON fields.
    pub fn from_fields(
        is_prime: bool,
        vaulted: Option<bool>,
        vault_date: Option<&str>,
        estimated_vault_date: Option<&str>,
    ) -> Self {
        if !is_prime {
            return VaultStatus::NotPrime;
        }

        match (vaulted, vault_date, estimated_vault_date) {
            (Some(true), Some(date), _) => VaultStatus::Vaulted {
                date: date.to_string(),
            },
            (Some(true), None, _) => VaultStatus::Vaulted {
                date: String::new(),
            },
            (_, _, Some(est_date)) if !vaulted.unwrap_or(false) => VaultStatus::EstimatedVault {
                estimated_date: est_date.to_string(),
            },
            _ => VaultStatus::Active,
        }
    }

    /// Check if this is a Prime item (any status except NotPrime)
    pub fn is_prime(&self) -> bool {
        !matches!(self, VaultStatus::NotPrime)
    }

    /// Check if the item is currently vaulted
    pub fn is_vaulted(&self) -> bool {
        matches!(self, VaultStatus::Vaulted { .. })
    }

    /// Check if the item is accessible (not vaulted)
    pub fn is_accessible(&self) -> bool {
        !self.is_vaulted()
    }

    /// Get the vault date if vaulted
    pub fn vault_date(&self) -> Option<&str> {
        match self {
            VaultStatus::Vaulted { date } if !date.is_empty() => Some(date),
            _ => None,
        }
    }

    /// Get the estimated vault date if applicable
    pub fn estimated_vault_date(&self) -> Option<&str> {
        match self {
            VaultStatus::EstimatedVault { estimated_date } => Some(estimated_date),
            _ => None,
        }
    }
}

impl Trigger {
    /// Convert to string representation for trait implementations
    pub fn as_str(&self) -> &'static str {
        match self {
            Trigger::Active => "Active",
            Trigger::Auto => "Auto",
            Trigger::AutoBurst => "Auto Burst",
            Trigger::Burst => "Burst",
            Trigger::Charge => "Charge",
            Trigger::Duplex => "Duplex",
            Trigger::Held => "Held",
            Trigger::Melee => "Melee",
            Trigger::Semi => "Semi",
            Trigger::Unknown => "Unknown",
        }
    }
}

impl Noise {
    /// Convert to string representation for trait implementations
    pub fn as_str(&self) -> &'static str {
        match self {
            Noise::Alarming => "Alarming",
            Noise::Silent => "Silent",
            Noise::Unknown => "Unknown",
        }
    }
}

impl Polarity {
    /// Convert to lowercase string representation (matches JSON format)
    pub fn as_str(&self) -> &'static str {
        match self {
            Polarity::Aura => "aura",
            Polarity::Madurai => "madurai",
            Polarity::Naramon => "naramon",
            Polarity::Penjaga => "penjaga",
            Polarity::Umbra => "umbra",
            Polarity::Unairu => "unairu",
            Polarity::Universal => "universal",
            Polarity::Vazarin => "vazarin",
            Polarity::Zenurik => "zenurik",
            Polarity::Any => "any",
            Polarity::Unknown => "unknown",
        }
    }
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

/// Mod category classification.
///
/// This is a computed enum derived from mod field presence.
/// Mods can be categorized as:
/// - Riven: has challenge/upgrade data for unveiling
/// - SetMember: belongs to a mod set (has modSet reference)
/// - SetDefinition: defines a mod set's bonuses (has numUpgradesInSet)
/// - Regular: standard mod with level stats
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum ModCategory {
    /// Riven mod with unveiling challenges
    Riven,
    /// Member of a mod set (references a set definition)
    SetMember {
        /// Reference to the set definition's unique name
        mod_set: String,
    },
    /// Mod set definition (describes set bonuses)
    SetDefinition {
        /// Number of mods in the set
        num_upgrades_in_set: i64,
    },
    /// Regular mod with level-based stats
    #[default]
    Regular,
}

impl ModCategory {
    /// Check if this is a Riven mod
    pub fn is_riven(&self) -> bool {
        matches!(self, ModCategory::Riven)
    }

    /// Check if this is part of a set (either member or definition)
    pub fn is_set(&self) -> bool {
        matches!(
            self,
            ModCategory::SetMember { .. } | ModCategory::SetDefinition { .. }
        )
    }

    /// Check if this is a set member
    pub fn is_set_member(&self) -> bool {
        matches!(self, ModCategory::SetMember { .. })
    }

    /// Check if this is a set definition
    pub fn is_set_definition(&self) -> bool {
        matches!(self, ModCategory::SetDefinition { .. })
    }

    /// Check if this is a regular mod
    pub fn is_regular(&self) -> bool {
        matches!(self, ModCategory::Regular)
    }

    /// Get the mod set reference if this is a set member
    pub fn mod_set(&self) -> Option<&str> {
        match self {
            ModCategory::SetMember { mod_set } => Some(mod_set),
            _ => None,
        }
    }

    /// Get the number of upgrades if this is a set definition
    pub fn num_upgrades_in_set(&self) -> Option<i64> {
        match self {
            ModCategory::SetDefinition {
                num_upgrades_in_set,
            } => Some(*num_upgrades_in_set),
            _ => None,
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

    #[test]
    fn test_vault_status_not_prime() {
        let status = VaultStatus::from_fields(false, None, None, None);
        assert_eq!(status, VaultStatus::NotPrime);
        assert!(!status.is_prime());
        assert!(!status.is_vaulted());
        assert!(status.is_accessible());
    }

    #[test]
    fn test_vault_status_active_prime() {
        let status = VaultStatus::from_fields(true, Some(false), None, None);
        assert_eq!(status, VaultStatus::Active);
        assert!(status.is_prime());
        assert!(!status.is_vaulted());
        assert!(status.is_accessible());
    }

    #[test]
    fn test_vault_status_vaulted_prime() {
        let status = VaultStatus::from_fields(true, Some(true), Some("2021-09-08"), None);
        assert!(matches!(status, VaultStatus::Vaulted { .. }));
        assert!(status.is_prime());
        assert!(status.is_vaulted());
        assert!(!status.is_accessible());
        assert_eq!(status.vault_date(), Some("2021-09-08"));
    }

    #[test]
    fn test_vault_status_estimated_vault() {
        let status = VaultStatus::from_fields(true, Some(false), None, Some("2023-03-14"));
        assert!(matches!(status, VaultStatus::EstimatedVault { .. }));
        assert!(status.is_prime());
        assert!(!status.is_vaulted());
        assert!(status.is_accessible());
        assert_eq!(status.estimated_vault_date(), Some("2023-03-14"));
    }

    #[test]
    fn test_mod_category_regular() {
        let cat = ModCategory::Regular;
        assert!(cat.is_regular());
        assert!(!cat.is_riven());
        assert!(!cat.is_set());
        assert!(cat.mod_set().is_none());
        assert!(cat.num_upgrades_in_set().is_none());
    }

    #[test]
    fn test_mod_category_riven() {
        let cat = ModCategory::Riven;
        assert!(cat.is_riven());
        assert!(!cat.is_regular());
        assert!(!cat.is_set());
    }

    #[test]
    fn test_mod_category_set_member() {
        let cat = ModCategory::SetMember {
            mod_set: "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod".to_string(),
        };
        assert!(cat.is_set());
        assert!(cat.is_set_member());
        assert!(!cat.is_set_definition());
        assert!(!cat.is_regular());
        assert_eq!(
            cat.mod_set(),
            Some("/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod")
        );
    }

    #[test]
    fn test_mod_category_set_definition() {
        let cat = ModCategory::SetDefinition {
            num_upgrades_in_set: 3,
        };
        assert!(cat.is_set());
        assert!(cat.is_set_definition());
        assert!(!cat.is_set_member());
        assert!(!cat.is_regular());
        assert_eq!(cat.num_upgrades_in_set(), Some(3));
    }
}

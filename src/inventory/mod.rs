use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Warframe frame module
pub mod suite;

/// Warframe primary weapon module
pub mod long_gun;

/// Warframe secondary weapon module
pub mod pistol;

/// Warframe upgrade (mods) module
pub mod upgrades;

#[derive(Debug, Serialize, Deserialize)]
pub enum FractionSyndicates {
    SteelMeridianSyndicate,
    ArbitersSyndicate,
    CephalonSudaSyndicate,
    PerrinSyndicate,
    RedVeilSyndicate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    /// Warframes
    #[serde(rename = "Suits")]
    pub suits: Vec<suite::Suit>,

    /// Primary Weapons
    #[serde(rename = "LongGuns")]
    pub long_guns: Vec<long_gun::LongGun>,

    /// Secondary Weapons
    #[serde(rename = "Pistols")]
    pub pistols: Vec<pistol::Pistol>,

    /// Mods + Arcanes (unupgraded)
    #[serde(rename = "RawUpgrades")]
    pub raw_upgrades: Vec<upgrades::RawUpgrade>,

    /// Mods + Arcanes (upgraded)
    #[serde(rename = "Upgrades")]
    pub upgrades: Vec<upgrades::Upgrade>,

    /// Player remaining trades for the day
    #[serde(rename = "TradesRemaining")]
    pub trades_remaining: Option<i64>,

    /// Syndicate
    #[serde(rename = "SupportedSyndicate")]
    pub supported_syndicates: Option<FractionSyndicates>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_inventory_deserialize() {
        let inventory_str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/sample_inventory.json"
        ));
        let inventory: Inventory =
            from_str(inventory_str).unwrap();
        assert!(!inventory.suits.is_empty(), "Suits should not be empty");
        assert!(
            !inventory.long_guns.is_empty(),
            "Long guns should not be empty"
        );
        assert!(!inventory.pistols.is_empty(), "Pistols should not be empty");
        assert!(
            !inventory.raw_upgrades.is_empty(),
            "Upgrades should not be empty"
        );
    }
}

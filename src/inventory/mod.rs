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
pub struct Inventory {
    #[serde(rename = "Suits")]
    pub suits: Vec<suite::Suit>,

    #[serde(rename = "LongGuns")]
    pub long_guns: Vec<long_gun::LongGun>,

    #[serde(rename = "Pistols")]
    pub pistols: Vec<pistol::Pistol>,

    #[serde(rename = "RawUpgrades")]
    pub upgrades: Vec<upgrades::RawUpgrade>,

    #[serde(rename = "Upgrades")]
    pub upgraded_mods: Vec<upgrades::Upgrade>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

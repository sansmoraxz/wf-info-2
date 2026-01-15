use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Warframe frame module
pub mod suite;

pub mod long_gun;

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(rename = "Suits")]
    pub suits: Vec<suite::Suit>,

    #[serde(rename = "LongGuns")]
    pub long_guns: Vec<long_gun::LongGun>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

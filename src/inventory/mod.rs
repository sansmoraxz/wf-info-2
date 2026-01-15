use serde::{Deserialize, Serialize};

pub mod suite;

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(rename = "Suits")]
    pub suits: Vec<suite::Suit>,
}

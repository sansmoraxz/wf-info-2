use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod arcane;

#[derive(Debug, Serialize, Deserialize)]

pub struct PatchLog {
    pub name: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub url: String,
    #[serde(rename = "imgUrl")]
    pub img_url: Option<String>,
    pub additions: String,
    pub changes: String,
    pub fixes: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Update {
    name: String,
    url: String,
    aliases: Vec<String>,
    parent: String,
    date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DropChance {
    pub chance: f32,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelStats {
    pub stats: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Polarity {
    #[serde(rename = "aura")]
    Aura,

    #[serde(rename = "madurai")]
    Madurai,

    #[serde(rename = "naramon")]
    Naramon,

    #[serde(rename = "penjaga")]
    Penjaga,

    #[serde(rename = "umbra")]
    Umbra,

    #[serde(rename = "unairu")]
    Unairu,

    #[serde(rename = "universal")]
    Universal,

    #[serde(rename = "vazarin")]
    Vazarin,

    #[serde(rename = "zenurik")]
    Zenurik,

    #[serde(rename = "any")]
    Any,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Trigger {
    #[serde(rename = "Active")]
    Active,

    #[serde(rename = "Auto")]
    Auto,

    #[serde(rename = "Auto Burst")]
    AutoBurst,

    #[serde(rename = "Burst")]
    Burst,

    #[serde(rename = "Charge")]
    Charge,

    #[serde(rename = "Duplex")]
    Duplex,

    #[serde(rename = "Held")]
    Held,

    #[serde(rename = "Melee")]
    Melee,

    #[serde(rename = "Semi")]
    Semi,

    #[serde(rename = "")]
    Unknown,
}

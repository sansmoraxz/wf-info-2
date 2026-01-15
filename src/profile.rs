use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    #[serde(rename = "Results")]
    pub results: Vec<ResultItem>,
    #[serde(rename = "Stats")]
    pub stats: Option<Stats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultItem {
    #[serde(rename = "AccountId")]
    pub account_id: AccountId,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "PlatformNames")]
    pub platform_names: Option<Vec<String>>,
    #[serde(rename = "PlayerLevel")]
    pub player_level: Option<f64>,
    #[serde(rename = "GuildName")]
    pub guild_name: Option<String>,
    #[serde(rename = "GuildTier")]
    pub guild_tier: Option<f64>,
    #[serde(rename = "GuildXp")]
    pub guild_xp: Option<f64>,
    #[serde(rename = "GuildClass")]
    pub guild_class: Option<f64>,
    #[serde(rename = "GuildEmblem")]
    pub guild_emblem: Option<bool>,
    // There are many more fields in LoadOutPreset and LoadOutInventory
    // Adding basics for now
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountId {
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "MissionsCompleted")]
    pub missions_completed: Option<f64>,
    #[serde(rename = "MissionsQuit")]
    pub missions_quit: Option<f64>,
    #[serde(rename = "MissionsFailed")]
    pub missions_failed: Option<f64>,
    #[serde(rename = "TimePlayedSec")]
    pub time_played_sec: Option<f64>,
    #[serde(rename = "Income")]
    pub income: Option<f64>,
}

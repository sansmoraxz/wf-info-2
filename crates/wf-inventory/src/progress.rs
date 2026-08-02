use crate::ItemType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents syndicate/faction standing.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Affiliation {
    #[serde(rename = "Tag")]
    pub tag: String,

    #[serde(rename = "Standing")]
    pub standing: Option<i64>,

    #[serde(rename = "Title")]
    pub title: Option<i64>,

    #[serde(rename = "FreeFavorsEarned")]
    pub free_favors_earned: Option<Vec<Value>>,

    #[serde(rename = "FreeFavorsUsed")]
    pub free_favors_used: Option<Vec<Value>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents mission completion data.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mission {
    #[serde(rename = "Tag")]
    pub tag: String,

    #[serde(rename = "Completes")]
    pub completes: Option<i64>,

    #[serde(rename = "Tier")]
    pub tier: Option<i64>,
}

/// Represents Nightwave/challenge progress.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeProgressEntry {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Progress")]
    pub progress: Option<Value>,
}

/// Represents lore fragment scan progress.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoreFragmentScan {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "Progress")]
    pub progress: Option<i64>,

    #[serde(rename = "Region")]
    pub region: Option<String>,
}

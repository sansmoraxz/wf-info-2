use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::ObjectId;

/// Represents an Operator loadout configuration.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperatorLoadOut {
    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "Skins")]
    pub skins: Option<Vec<String>>,

    #[serde(rename = "Upgrades")]
    pub upgrades: Option<Vec<String>>,

    #[serde(rename = "OperatorAmp")]
    pub operator_amp: Option<Value>,

    #[serde(rename = "AbilityOverride")]
    pub ability_override: Option<Value>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represents an Adult Operator (Drifter) loadout configuration.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdultOperatorLoadOut {
    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "Skins")]
    pub skins: Option<Vec<String>>,

    #[serde(rename = "Upgrades")]
    pub upgrades: Option<Vec<String>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

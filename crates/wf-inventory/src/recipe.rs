use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DateWrapper, ItemType, ObjectId};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemCount")]
    pub item_count: i64,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingRecipe {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "ItemCount", skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,

    #[serde(rename = "TargetItemId")]
    pub target_item_id: Option<String>,

    #[serde(rename = "CompletionDate")]
    pub completion_date: DateWrapper,

    #[serde(flatten)]
    pub other: Option<Value>,
}

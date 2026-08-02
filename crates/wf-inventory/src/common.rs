use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ItemType, ObjectId};

/// Simple countable item (Consumables, MiscItems, ShipDecorations, LevelKeys, etc.)
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountableItem {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemCount")]
    pub item_count: i64,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Type-only item (FlavourItems, FocusUpgrades, etc.)
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeOnlyItem {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Weapon skin reference (ItemType + ItemId)
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponSkin {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Inventory slot bin (Extra + Slots)
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotBin {
    #[serde(rename = "Extra")]
    pub extra: Option<i64>,

    #[serde(rename = "Slots")]
    pub slots: Option<i64>,
}

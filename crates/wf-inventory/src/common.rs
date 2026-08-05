use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ItemType, ObjectId, Polarity};

/// Simple countable item (Consumables, MiscItems, ShipDecorations, LevelKeys, etc.)
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountableItem {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemCount")]
    pub item_count: i64,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

/// Type-only item (FlavourItems, FocusUpgrades, etc.)
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeOnlyItem {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

/// Weapon skin reference (ItemType + ItemId)
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponSkin {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

/// Inventory slot bin (Extra + Slots)
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotBin {
    #[serde(rename = "Extra")]
    pub extra: Option<i64>,

    #[serde(rename = "Slots")]
    pub slots: Option<i64>,
}

/// Weapon-style entry: Melee, LongGuns, Pistols, SpaceSuits, SpaceGuns,
/// SpaceMelee.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponEntry {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "ItemCount", skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

    #[serde(rename = "FocusLens")]
    pub focus_lens: Option<String>,

    #[serde(rename = "Polarity")]
    pub polarity: Option<Vec<Polarity>>,

    #[serde(rename = "Polarized")]
    pub polarized: Option<i64>,

    #[serde(rename = "ModSlotPurchases")]
    pub mod_slot_purchases: Option<i64>,

    #[serde(rename = "IsNew")]
    pub is_new: Option<bool>,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

/// Configurable entry without XP: Antiques, DrifterMelee, Horses,
/// Motorcycles, OperatorSuits, Scoops.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigurableEntry {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(rename = "Configs")]
    pub configs: Option<Vec<Value>>,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

/// Configurable entry with XP: Sentinels, SentinelWeapons, DataKnives.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XpEntry {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(rename = "Configs")]
    pub configs: Option<Vec<Value>>,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

/// [`XpEntry`] plus modular parts: Hoverboards, MoaPets.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModularEntry {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(rename = "Configs")]
    pub configs: Option<Vec<Value>>,

    #[serde(rename = "ModularParts")]
    pub modular_parts: Option<Vec<String>>,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

/// Polarizable configurable entry: MechSuits, CrewShipHarnesses.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolarizedEntry {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

    #[serde(rename = "Features")]
    pub features: Option<i64>,

    #[serde(rename = "Polarity")]
    pub polarity: Option<Vec<Polarity>>,

    #[serde(rename = "Polarized")]
    pub polarized: Option<i64>,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(rename = "Configs")]
    pub configs: Option<Vec<Value>>,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

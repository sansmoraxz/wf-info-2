use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DateWrapper, ItemType, ObjectId, Polarity};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchonCrystalUpgrade {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "UpgradeType")]
    pub upgrade_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchonCrystalUpgradeWrapper {
    ArchonCrystalUpgrade(ArchonCrystalUpgrade),
    Array(Vec<Value>), // sometimes empty values are populated for offset
}

/// Represents a warframe suit in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Suit {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "ItemCount", skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,

    #[serde(rename = "InfestationDate")]
    pub infestation_date: Option<DateWrapper>,

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

    #[serde(rename = "ArchonCrystalUpgrades")]
    pub archon_crystal_upgrades: Option<Vec<ArchonCrystalUpgradeWrapper>>,

    #[serde(rename = "IsNew")]
    pub is_new: Option<bool>,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_inventory_suit() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_suit_test.json"
        ));

        let suit: Suit = from_str(json_data).unwrap();

        assert_eq!(suit.item_type, "/Lotus/Powersuits/Trinity/TrinityPrime");
        assert_eq!(suit.xp.unwrap(), 3_106_125);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The flatten catch-all is a string-keyed Map, so numeric-looking JSON
    /// keys are plain strings and must survive the roundtrip. (Flatten only
    /// breaks on maps with non-string key types, which buffer through
    /// serde's internal representation.)
    #[test]
    fn numeric_looking_keys_roundtrip_through_map_catch_all() {
        let raw = r#"{"ItemType":"/Lotus/Test","ItemId":{"$oid":"abc"},"123":45,"":"empty-key","9.5":[1,2]}"#;
        let suit: Suit = serde_json::from_str(raw).unwrap();
        let other = suit.other.as_ref().unwrap();
        assert_eq!(other.get("123").unwrap(), 45_i32);
        assert_eq!(other.get("").unwrap(), "empty-key");
        assert!(other.get("9.5").unwrap().is_array());
        let back = serde_json::to_value(&suit).unwrap();
        assert_eq!(back["123"], 45_i32);
        assert_eq!(back["9.5"], serde_json::json!([1_i32, 2_i32]));
    }

    /// Unquoted keys (`{1:"hello"}`) are invalid JSON: rejected by the
    /// parser itself, identically for a `Map` catch-all and the previous
    /// `Value` one.
    #[test]
    fn unquoted_numeric_key_is_a_parse_error_regardless_of_catch_all_type() {
        let raw = r#"{"ItemType":"/Lotus/Test","ItemId":{"$oid":"abc"},1:"hello"}"#;
        serde_json::from_str::<Suit>(raw).unwrap_err();
        serde_json::from_str::<serde_json::Value>(raw).unwrap_err();
    }
}

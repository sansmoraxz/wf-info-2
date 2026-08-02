use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DateWrapper, ObjectId, Polarity};

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
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

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
    pub other: Option<Value>,
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

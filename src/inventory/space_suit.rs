use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::{ObjectId, Polarity};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpaceSuit {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

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
    pub other: Option<Value>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_inventory_space_suit() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_space_suit_test.json"
        ));

        let space_suit: SpaceSuit = from_str(json_data).unwrap();

        assert_eq!(space_suit.item_type, "/Lotus/Powersuits/Archwing/SupportJetPack/SupportJetPack");
        assert_eq!(space_suit.xp.unwrap(), 4376023);
    }
}

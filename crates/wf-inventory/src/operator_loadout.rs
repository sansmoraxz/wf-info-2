use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ObjectId;

/// Represents an Operator loadout configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_operator_loadout() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_operator_loadout_test.json"
        ));

        let item: OperatorLoadOut = from_str(json_data).unwrap();
        assert_eq!(item.item_id.as_ref(), "000000000000000000000000");
    }

    #[test]
    fn test_deserialize_adult_operator_loadout() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_adult_operator_loadout_test.json"
        ));

        let item: AdultOperatorLoadOut = from_str(json_data).unwrap();
        assert_eq!(item.item_id.as_ref(), "618d769e3348adda0fc130ae");
    }
}

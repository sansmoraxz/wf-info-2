use serde::{Deserialize, Serialize};

use crate::common::XpEntry;

/// Represents a sentinel companion in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Sentinel(pub XpEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_sentinel() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_sentinel_test.json"
        ));

        let item: Sentinel = from_str(json_data).unwrap();

        assert_eq!(
            item.0.item_type,
            "/Lotus/Types/Sentinels/SentinelPowersuits/TnSentinelCrossPowerSuit"
        );
        assert_eq!(item.0.xp.unwrap(), 18_583_058);
    }
}

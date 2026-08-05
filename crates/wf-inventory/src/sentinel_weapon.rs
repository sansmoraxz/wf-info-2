use serde::{Deserialize, Serialize};

use crate::common::XpEntry;

/// Represents a sentinel weapon in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SentinelWeapon(pub XpEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_sentinelweapon() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_sentinel_weapon_test.json"
        ));

        let item: SentinelWeapon = from_str(json_data).unwrap();

        assert_eq!(
            item.0.item_type,
            "/Lotus/Types/Sentinels/SentinelWeapons/Gremlin"
        );
        assert_eq!(item.0.xp.unwrap(), 20_545_526);
    }
}

use serde::{Deserialize, Serialize};

use crate::common::ModularEntry;

/// Represents a K-Drive (hoverboard) in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Hoverboard(pub ModularEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_hoverboard() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_hoverboard_test.json"
        ));

        let item: Hoverboard = from_str(json_data).unwrap();

        assert_eq!(
            item.0.item_type,
            "/Lotus/Types/Vehicles/Hoverboard/HoverboardSuit"
        );
        assert_eq!(item.0.xp.unwrap(), 1_365_967);
    }
}

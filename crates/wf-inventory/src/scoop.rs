use serde::{Deserialize, Serialize};

use crate::common::ConfigurableEntry;

/// Represents a scoop (void energy collector) in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Scoop(pub ConfigurableEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_scoop() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_scoop_test.json"
        ));

        let item: Scoop = from_str(json_data).unwrap();

        assert_eq!(
            item.0.item_type,
            "/Lotus/Weapons/Tenno/Speedball/SpeedballWeaponTest"
        );
    }
}

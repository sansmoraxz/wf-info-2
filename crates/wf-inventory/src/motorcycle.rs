use serde::{Deserialize, Serialize};

use crate::common::ConfigurableEntry;

/// Represents a motorcycle in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Motorcycle(pub ConfigurableEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_motorcycle() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_motorcycle_test.json"
        ));

        let item: Motorcycle = from_str(json_data).unwrap();

        assert_eq!(
            item.0.item_type,
            "/Lotus/Types/Vehicles/Motorcycle/MotorcyclePowerSuit"
        );
    }
}

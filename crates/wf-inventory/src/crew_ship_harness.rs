use serde::{Deserialize, Serialize};

use crate::common::PolarizedEntry;

/// Represents a Railjack reactor/harness in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CrewShipHarness(pub PolarizedEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_crewshipharness() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_crew_ship_harness_test.json"
        ));

        let item: CrewShipHarness = from_str(json_data).unwrap();

        assert_eq!(
            item.0.item_type,
            "/Lotus/Types/Game/CrewShip/RailJack/DefaultHarness"
        );
        assert_eq!(item.0.xp.unwrap(), 21_375_974);
    }
}

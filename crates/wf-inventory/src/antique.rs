use serde::{Deserialize, Serialize};

use crate::common::ConfigurableEntry;

/// Represents an antique (Operator weapon) in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Antique(pub ConfigurableEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_antique() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_antique_test.json"
        ));

        let item: Antique = from_str(json_data).unwrap();

        assert_eq!(
            item.0.item_type,
            "/Lotus/Weapons/Operator/Antiques/MaduraiBow/MaduraiBowAntique"
        );
    }
}

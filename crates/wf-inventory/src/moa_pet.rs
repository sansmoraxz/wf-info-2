use serde::{Deserialize, Serialize};

use crate::common::ModularEntry;

/// Represents a MOA companion in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MoaPet(pub ModularEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_moapet() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_moa_pet_test.json"
        ));

        let item: MoaPet = from_str(json_data).unwrap();

        assert_eq!(
            item.0.item_type,
            "/Lotus/Types/Friendly/Pets/MoaPets/MoaPetPowerSuit"
        );
        assert_eq!(item.0.xp.unwrap(), 904_219);
    }
}

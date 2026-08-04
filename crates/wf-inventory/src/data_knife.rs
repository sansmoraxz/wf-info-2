use serde::{Deserialize, Serialize};

use crate::common::XpEntry;

/// Represents a data knife (hacking device / parazon) in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataKnife(pub XpEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_dataknife() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_data_knife_test.json"
        ));

        let item: DataKnife = from_str(json_data).unwrap();

        assert_eq!(
            item.0.item_type,
            "/Lotus/Weapons/Tenno/HackingDevices/TnHackingDevice/TnHackingDeviceWeapon"
        );
        assert_eq!(item.0.xp.unwrap(), 450_000);
    }
}

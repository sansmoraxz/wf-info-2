use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::{DateWrapper, ObjectId, Polarity};

/// Represents a warframe suit in the inventory.
#[derive(Debug, Serialize, Deserialize)]
pub struct Suit {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "InfestationDate")]
    pub infestation_date: Option<DateWrapper>,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

    #[serde(rename = "FocusLens")]
    pub focus_lens: Option<String>,

    #[serde(rename = "Polarity")]
    pub polarity: Option<Vec<Polarity>>,

    #[serde(rename = "Polarized")]
    pub polarized: Option<i64>,

    #[serde(rename = "ModSlotPurchases")]
    pub mod_slot_purchases: Option<i64>,

    #[serde(rename = "ArchonCrystalUpgrades")]
    pub archon_crystal_upgrades: Option<Vec<Value>>,

    #[serde(rename = "IsNew")]
    pub is_new: Option<bool>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_inventory_suite() {
        let json_data = r#"
{
  "Configs": [
    {
      "Skins": [
        "/Lotus/Upgrades/Skins/Gyre/GyrePrimeHelmet",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/GyrePrimeArmArmor",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "",
        "",
        "/Lotus/Upgrades/Skins/Gyre/GyreNobleAnims",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Gyre/GyreSkin",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/GyrePrimeArmArmor",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "",
        "",
        "",
        "",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "/Lotus/Upgrades/Skins/Voices/DefaultWarframeVoiceItem"
      ],
      "Upgrades": [
        "5beac8543f8d4a639e64fd21",
        "6967ce67ad4d4af97c0885d0",
        "69669c5cffa7b9b95f0c81fd",
        "65579f4c5a06b05cbd089743",
        "5f6dea663fb20d1a4137422e",
        "69669c7ff9306e22ff09cc22",
        "69669bf27f304f16f30dd927",
        "612b83281f37f56af164adaf",
        "5bf42943a38e4a229f12e11c",
        "5f65c50984a6266c1d69670e",
        "6295bd5ce8601e1c99263f3d",
        "628e925ce273271e121fc2d1"
      ],
      "pricol": {
        "t3": -6907494
      }
    },
    {
      "Skins": [
        "696876929d69a047c007b210",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/GyrePrimeArmArmor",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "",
        "",
        "/Lotus/Upgrades/Skins/Gyre/GyreNobleAnims",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "696876929d69a047c007b20f",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/GyrePrimeArmArmor",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "",
        "",
        "",
        "",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "/Lotus/Upgrades/Skins/Voices/DefaultWarframeVoiceItem"
      ]
    },
    {
      "Skins": [
        "/Lotus/Upgrades/Skins/Gyre/GyrePrimeHelmet",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/GyrePrimeArmArmor",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "",
        "",
        "/Lotus/Upgrades/Skins/Gyre/GyreNobleAnims",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Gyre/GyrePrimeSkin",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/GyrePrimeArmArmor",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization"
      ]
    }
  ],
  "Features": 3,
  "FocusLens": "/Lotus/Upgrades/Focus/WardLensOstron",
  "InfestationDate": {
    "$date": {
      "$numberLong": "2147483647000"
    }
  },
  "ItemId": {
    "$oid": "695a3cea900c37cf8c107e47"
  },
  "ItemType": "/Lotus/Powersuits/Gyre/GyrePrime",
  "Polarity": [
    {
      "Slot": 6,
      "Value": "AP_TACTIC"
    },
    {
      "Slot": 1,
      "Value": "AP_ATTACK"
    }
  ],
  "Polarized": 2,
  "UpgradeVer": 101,
  "XP": 420021
}"#;

        let suit: Suit = from_str(json_data).unwrap();

        assert_eq!(suit.item_type, "/Lotus/Powersuits/Gyre/GyrePrime");
        assert_eq!(suit.xp.unwrap(), 420021);
    }
}

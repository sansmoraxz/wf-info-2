use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::{DateWrapper, ObjectId, Polarity};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchonCrystalUpgrade {
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "UpgradeType")]
    pub upgrade_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchonCrystalUpgradeWrapper {
    ArchonCrystalUpgrade(ArchonCrystalUpgrade),
    Array(Vec<Value>), // sometimes empty values are populated for offset
}

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
    pub archon_crystal_upgrades: Option<Vec<ArchonCrystalUpgradeWrapper>>,

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
  "ItemType": "/Lotus/Powersuits/Trinity/TrinityPrime",
  "Configs": [
    {
      "Skins": [
        "5df0c07a3f8d4a16212f205f",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "610689965bc99c253e5b3246",
        "",
        "",
        "/Lotus/Upgrades/Skins/Trinity/TrinityNobleAnims",
        "60d5c876b91db76455665981",
        "/Lotus/Upgrades/Skins/Trinity/TrinityPrimeSkin",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "60ffb32918214a280e10d355",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "",
        "",
        "",
        "/Lotus/Upgrades/Skins/Trinity/TrinityEffectsSetDefault",
        "60e82703c975c96e02122634",
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
      "sigcol": {
        "t1": 13115392,
        "t2": -2145378765,
        "t3": 8388608,
        "m1": -2144589250,
        "en": 127
      },
      "pricol": {
        "t0": -14934241,
        "t1": -2236706,
        "t2": -2228224,
        "t3": -11831684,
        "m0": -3068365,
        "en": -11289495
      },
      "attcol": {
        "t0": -787462,
        "t1": -9388879,
        "t2": -12949153,
        "t3": -1,
        "m0": -3068365,
        "m1": -36352,
        "en": -4269153
      },
      "Name": "Avenging Angel",
      "syancol": {
        "t0": -62452,
        "t1": -2236706,
        "t2": -15197670,
        "t3": -11831684,
        "m0": -3068365,
        "m1": -3068365,
        "en": -11289495
      },
      "Upgrades": [
        "62b8895f177cd730342c7dda",
        "5f6733903e6f3a6d9668ebd9",
        "60e453e294d3bd7ac20c8700",
        "612707449dbe520e420f3de9",
        "6963b685666893fc760f85b4",
        "612b83281f37f56af164adaf",
        "611504de44eb235ab631dbd5",
        "5fb192b598bd2f70082fd358",
        "5bea8de2a38e4ab6f123bf53",
        "5f65c50984a6266c1d69670e",
        "5e7115085ca4142ad905f3ef",
        "6963b60fda8a5ab48c082f87"
      ]
    },
    {
      "Skins": [
        "5df0c07a3f8d4a16212f205f",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "",
        "",
        "/Lotus/Upgrades/Skins/Trinity/TrinityNobleAnims",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Trinity/TrinityPrimeSkin",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "",
        "",
        "",
        "/Lotus/Upgrades/Skins/Trinity/TrinityEffectsSetDefault",
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
      "Name": "spread out",
      "Upgrades": [
        "62b8895f177cd730342c7dda",
        "5f6a5ad2079a17549923523a",
        "60e453e294d3bd7ac20c8700",
        "62db829b3221647b3c070555",
        "5bea8e7257904aa1056299d2",
        "5bfd01e457904abd35638c3f",
        "612b83281f37f56af164adaf",
        "5fb192b598bd2f70082fd358",
        "5bea8de2a38e4ab6f123bf53",
        "/Lotus/Upgrades/Mods/Sets/Nira/NiraExilusMod",
        "5dfcb1bba38e4ab2e00e8f4b",
        "6159f0c72968a2319429708b"
      ]
    },
    {
      "Skins": [
        "5df0c07a3f8d4a16212f205f",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "610689965bc99c253e5b3246",
        "",
        "",
        "/Lotus/Upgrades/Skins/Trinity/TrinityNobleAnims",
        "60d5c876b91db76455665981",
        "5f7c2f1f1b33c62a815eb199",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "60ffb32918214a280e10d355",
        "/Lotus/Upgrades/Skins/Armor/WarframeDefaults/EmptyCustomization",
        "",
        "",
        "",
        "/Lotus/Upgrades/Skins/Trinity/TrinityEffectsSetDefault",
        "60e82703c975c96e02122634",
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
      "Name": "roar",
      "AbilityOverride": {
        "Ability": "/Lotus/Powersuits/PowersuitAbilities/RhinoRoarAbility",
        "Index": 0
      },
      "attcol": {
        "t0": -787462,
        "t1": -9388879,
        "t2": -12949153,
        "t3": -1,
        "m0": -3068365,
        "en": -4269153
      },
      "pricol": {
        "t0": -1,
        "t1": -14868958,
        "t2": -11286148,
        "t3": -9388879,
        "m0": -3068365,
        "en": -11289495
      },
      "ugly": true,
      "syancol": {
        "t0": -11157550,
        "t3": -3068365
      },
      "Upgrades": [
        "",
        "5bfd01e457904abd35638c3f",
        "60e453e294d3bd7ac20c8700",
        "5f65d8b9967a310c494dba66",
        "5fb646a8b5789b4b3e116a46",
        "5bfeb9eda38e4a76ec055d26",
        "612b83281f37f56af164adaf",
        "5fb192b598bd2f70082fd358",
        "5bea8de2a38e4ab6f123bf53",
        "628521f6a7aaf41fd51740fe",
        "6295d0a23cb7474c283b9343",
        "6116b1063292ca1d4c5d7778"
      ]
    },
    {
      "Name": "goldseeker",
      "Upgrades": [
        "",
        "5f6a5ad2079a17549923523a",
        "60e453e294d3bd7ac20c8700",
        "5be49b1a3f8d4a8c8f43b8d5",
        "5bea8e7257904aa1056299d2",
        "613ca6d6fbfedf22241d38be",
        "5bfd01e457904abd35638c3f",
        "5fb192b598bd2f70082fd358",
        "5bea8de2a38e4ab6f123bf53",
        "6287d0a9f2d0d231823940f5",
        "6116b1063292ca1d4c5d7778",
        "5f6dcdf0365fbb3eb2534adc"
      ],
      "Skins": [
        "/Lotus/Upgrades/Skins/Trinity/TrinityPrimeHelmet"
      ],
      "AbilityOverride": {
        "Ability": "/Lotus/Powersuits/PowersuitAbilities/HelminthTreasureAbility",
        "Index": 0
      }
    }
  ],
  "UpgradeVer": 101,
  "InfestationDate": {
    "$date": {
      "$numberLong": "2147483647000"
    }
  },
  "XP": 3106125,
  "Features": 3,
  "FocusLens": "/Lotus/Upgrades/Focus/PowerLens",
  "ModSlotPurchases": 1,
  "ArchonCrystalUpgrades": [
    [],
    {
      "Color": "ACC_YELLOW_MYTHIC",
      "UpgradeType": "/Lotus/Upgrades/Invigorations/ArchonCrystalUpgrades/ArchonCrystalUpgradeWarframeCastingSpeedMythic"
    }
  ],
  "Polarity": [
    {
      "Slot": 5,
      "Value": "AP_ATTACK"
    }
  ],
  "Polarized": 1,
  "ItemId": {
    "$oid": "5df0c07a3f8d4a16212f205e"
  }
}
"#;

        let suit: Suit = from_str(json_data).unwrap();

        assert_eq!(suit.item_type, "/Lotus/Powersuits/Trinity/TrinityPrime");
        assert_eq!(suit.xp.unwrap(), 3106125);
    }
}

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{PickFirst, Same, json::JsonString, serde_as};

use crate::{ItemType, ObjectId};

/// Represent unupgraded mods
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawUpgrade {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "LastAdded")]
    pub last_added_id: ObjectId,

    #[serde(rename = "ItemCount")]
    pub item_count: i64,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

/// Represent upgraded mods
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Upgrade {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "ItemCount", skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,

    /// The wire form is usually JSON embedded in a string, but bare objects also occur;
    /// serialization always emits the string form.
    #[serde(rename = "UpgradeFingerprint")]
    #[serde_as(as = "PickFirst<(JsonString, Same)>")]
    pub upgrade_fingerprint: UpgradeFingerprint,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassicFingerprint {
    pub lvl: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Buff {
    #[serde(rename = "Tag")]
    pub tag: String,

    #[serde(rename = "Value")]
    pub value: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RivenFingerprint {
    pub compat: String,
    pub lim: i64,
    #[serde(rename = "lvlReq")]
    pub lvl_req: Option<i64>,
    pub lvl: Option<i64>,
    pub rerolls: Option<i64>,
    pub pol: String,
    pub buffs: Vec<Buff>,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RivenChallengeDetail {
    #[serde(rename = "Type")]
    pub type_path: String,
    #[serde(rename = "Progress")]
    pub progress: i64,
    #[serde(rename = "Required")]
    pub required: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RivenChallenge {
    #[serde(rename = "challenge")]
    pub challenge: RivenChallengeDetail,
}

// Variant order matters: untagged tries top-down, and RivenChallenge/RivenMod have
// required fields that ClassicFingerprint's flexible shape would otherwise swallow.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpgradeFingerprint {
    /// Riven challenge for veiled riven mods
    RivenChallenge(RivenChallenge),
    /// Riven mod structure as an object
    RivenMod(RivenFingerprint),
    /// When the fingerprint is stored as a string containing JSON: "{\"lvl\":10}"
    ClassicObj(ClassicFingerprint),
    /// Garbage fallback for unrecognized fingerprint types
    Unknown(Value),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_raw_upgrade() {
        let js = r#"{ "ItemCount": 32, "ItemType": "/Lotus/Upgrades/Mods/Warframe/AvatarShieldMaxMod", "LastAdded": {"$oid":"695a535a4e4a24181e0bf2b8"} }"#;
        let u: RawUpgrade = from_str(js).unwrap();
        assert_eq!(u.item_count, 32);
    }

    #[test]
    fn test_deserialize_upgrade_classic_string() {
        let js = r#"
{
    "ItemId": {
    "$oid": "5bc1067d57904a037245c7b0"
    },
    "ItemType": "/Lotus/Upgrades/Mods/Warframe/AvatarShieldMaxMod",
    "UpgradeFingerprint": "{\"lvl\":10}"
}
"#;
        let u: Upgrade = from_str(js).unwrap();
        assert_eq!(
            u.item_type,
            "/Lotus/Upgrades/Mods/Warframe/AvatarShieldMaxMod"
        );

        if let UpgradeFingerprint::ClassicObj(c) = u.upgrade_fingerprint {
            assert_eq!(c.lvl, 10);
        } else {
            panic!("expected ClassicObj variant");
        }
    }

    #[test]
    fn test_deserialize_upgrade_riven_object() {
        let js = r#"
{
    "ItemId": {
    "$oid": "60f73b8103cbfb55fc3b1998"
    },
    "ItemType": "/Lotus/Upgrades/Mods/Randomized/LotusPistolRandomModRare",
    "UpgradeFingerprint": "{\"compat\":\"/Lotus/Weapons/Tenno/Pistol/Pistol\",\"lim\":773424723,\"lvlReq\":13,\"pol\":\"AP_TACTIC\",\"buffs\":[{\"Tag\":\"WeaponDamageAmountMod\",\"Value\":128098035},{\"Tag\":\"WeaponPunctureDepthMod\",\"Value\":483497619},{\"Tag\":\"WeaponProcTimeMod\",\"Value\":490036261}],\"curses\":[{\"Tag\":\"WeaponFireIterationsMod\",\"Value\":235369015}],\"rerolls\":5,\"lvl\":8}"
}
"#;
        let u: Upgrade = from_str(js).unwrap();
        let fp = u.upgrade_fingerprint;

        if let UpgradeFingerprint::RivenMod(r) = fp {
            assert_eq!(r.compat, "/Lotus/Weapons/Tenno/Pistol/Pistol");
            assert_eq!(r.lim, 773_424_723);
            assert_eq!(r.lvl_req, Some(13));
            assert_eq!(r.lvl, Some(8));
            assert_eq!(r.rerolls, Some(5));
            assert_eq!(r.buffs.len(), 3);
        } else {
            panic!("expected Riven variant");
        }
    }

    #[test]
    fn test_deserialize_upgrade_riven_object_2() {
        let js = r#"
{
    "ItemId": {
    "$oid": "629b688edf244f5a803d9a68"
    },
    "ItemType": "/Lotus/Upgrades/Mods/Randomized/PlayerMeleeWeaponRandomModRare",
    "UpgradeFingerprint": "{\"compat\":\"/Lotus/Weapons/Tenno/Melee/Swords/DarkSword/DarkLongSword\",\"lim\":380023905,\"lvlReq\":12,\"pol\":\"AP_DEFENSE\",\"buffs\":[{\"Tag\":\"WeaponMeleeFactionDamageGrineer\",\"Value\":949496221},{\"Tag\":\"WeaponFireDamageMod\",\"Value\":51975813}]}"
}
"#;
        let u: Upgrade = from_str(js).unwrap();
        let fp = u.upgrade_fingerprint;

        if let UpgradeFingerprint::RivenMod(r) = fp {
            assert_eq!(
                r.compat,
                "/Lotus/Weapons/Tenno/Melee/Swords/DarkSword/DarkLongSword"
            );
            assert_eq!(r.lim, 380_023_905);
            assert_eq!(r.lvl_req, Some(12));
            assert_eq!(r.pol, "AP_DEFENSE");
            assert_eq!(r.buffs.len(), 2);
        } else {
            panic!("expected Riven variant");
        }
    }

    #[test]
    fn test_deserialize_upgrade_riven_challenge() {
        let js = r#"
{
    "ItemId": {
    "$oid": "60fd88234f5be52ac1332f2c"
    },
    "ItemType": "/Lotus/Upgrades/Mods/Randomized/LotusShotgunRandomModRare",
    "UpgradeFingerprint": "{\"challenge\":{\"Type\":\"/Lotus/Types/Challenges/RandomizedFlyingHeadshotSeries\",\"Progress\":0,\"Required\":8}}"
}
        "#;
        let u: Upgrade = from_str(js).unwrap();
        let fp = u.upgrade_fingerprint;

        if let UpgradeFingerprint::RivenChallenge(rc) = fp {
            assert_eq!(
                rc.challenge.type_path,
                "/Lotus/Types/Challenges/RandomizedFlyingHeadshotSeries"
            );
            assert_eq!(rc.challenge.progress, 0);
            assert_eq!(rc.challenge.required, 8);
        } else {
            panic!("expected RivenChallenge variant");
        }
    }
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represent unupgraded mods
#[derive(Debug, Serialize, Deserialize)]
pub struct RawUpgrade {
    #[serde(rename = "ItemCount")]
    pub item_count: i64,

    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "LastAdded")]
    pub last_added: Option<ObjectId>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represent upgraded mods
#[derive(Debug, Serialize, Deserialize)]
pub struct Upgrade {
    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "UpgradeFingerprint")]
    pub upgrade_fingerprint: UpgradeFingerprint,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassicFingerprint {
    pub lvl: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Buff {
    #[serde(rename = "Tag")]
    pub tag: String,

    #[serde(rename = "Value")]
    pub value: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RivenFingerprint {
    pub compat: String,
    pub lim: i64,
    #[serde(rename = "lvlReq")]
    pub lvl_req: i64,
    pub lvl: i64,
    pub rerolls: i64,
    pub pol: String,
    pub buffs: Vec<Buff>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[derive(Debug, Clone)]
pub enum UpgradeFingerprint {
    /// When the fingerprint is stored as a string containing JSON: "{\"lvl\":10}"
    ClassicObj(ClassicFingerprint),
    /// Riven mod structure as an object
    Riven(RivenFingerprint),
}

impl serde::Serialize for UpgradeFingerprint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize the inner struct as a JSON string (to match server/client stringified form)
        let s = match self {
            UpgradeFingerprint::ClassicObj(c) => serde_json::to_string(c),
            UpgradeFingerprint::Riven(r) => serde_json::to_string(r),
        }
        .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for UpgradeFingerprint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // First deserialize into a serde_json::Value so we can handle both string and object forms.
        let v = serde_json::Value::deserialize(deserializer).map_err(serde::de::Error::custom)?;

        match v {
            serde_json::Value::String(s) => {
                // When a string is provided, it should be JSON text we must parse.
                // Try riven first, then classic.
                if let Ok(r) = serde_json::from_str::<RivenFingerprint>(&s) {
                    return Ok(UpgradeFingerprint::Riven(r));
                }
                if let Ok(c) = serde_json::from_str::<ClassicFingerprint>(&s) {
                    return Ok(UpgradeFingerprint::ClassicObj(c));
                }
                Err(serde::de::Error::custom(
                    "string fingerprint did not match known fingerprint shapes",
                ))
            }
            serde_json::Value::Object(_) => {
                // When an object is provided directly, try to deserialize into riven then classic.
                let v_clone = v.clone();
                if let Ok(r) = serde_json::from_value::<RivenFingerprint>(v_clone.clone()) {
                    return Ok(UpgradeFingerprint::Riven(r));
                }
                if let Ok(c) = serde_json::from_value::<ClassicFingerprint>(v_clone) {
                    return Ok(UpgradeFingerprint::ClassicObj(c));
                }
                Err(serde::de::Error::custom(
                    "object fingerprint did not match known fingerprint shapes",
                ))
            }
            _ => Err(serde::de::Error::custom(
                "unexpected type for UpgradeFingerprint; expected string or object",
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectId {
    #[serde(rename = "$oid")]
    pub oid: String,
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
    "$oid": "5bc1067d57904a037245c7b0"
    },
    "ItemType": "/Lotus/Upgrades/Mods/Weapon/RivenExample",
    "UpgradeFingerprint": {
        "compat": "/Lotus/Weapons/Infested/Pistols/InfKitGun/Barrels/InfBarrelEgg/InfModularBarrelEggPart",
        "lim": 1811314,
        "lvlReq": 11,
        "lvl": 8,
        "rerolls": 9,
        "pol": "AP_DEFENSE",
        "buffs": [
            { "Tag": "WeaponRecoilReductionMod", "Value": 119165508 },
            { "Tag": "WeaponDamageAmountMod", "Value": 202120506 },
            { "Tag": "WeaponStunChanceMod", "Value": 761824003 }
        ]
    }
}
"#;
        let u: Upgrade = from_str(js).unwrap();
        let fp = u.upgrade_fingerprint;

        if let UpgradeFingerprint::Riven(r) = fp {
            assert_eq!(
                r.compat,
                "/Lotus/Weapons/Infested/Pistols/InfKitGun/Barrels/InfBarrelEgg/InfModularBarrelEggPart"
            );
            assert_eq!(r.lim, 1811314);
            assert_eq!(r.lvl_req, 11);
            assert_eq!(r.lvl, 8);
            assert_eq!(r.rerolls, 9);
            assert_eq!(r.buffs.len(), 3);
        } else {
            panic!("expected Riven variant");
        }
    }
}

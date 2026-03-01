//! Enums for known finite value sets in item data.
//!
//! These replace String fields where the set of valid values is known and finite.
//! Each enum includes an `Unknown(String)` variant to gracefully handle new values
//! that may be added to the source data in the future, while preserving the original value.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Macro for generating string-backed enums with `Unknown(String)` fallback.
///
/// Each enum gets: `as_str()`, `Display`, custom `Serialize`/`Deserialize`,
/// and `Default` (either a named variant or `Unknown(String::new())`).
macro_rules! string_enum {
    // Default is Unknown
    ($(#[doc = $doc:expr])* pub enum $name:ident {
        default Unknown;
        $( $variant:ident => $str:expr ),* $(,)?
    }) => {
        string_enum!(@impl $(#[doc = $doc])* pub enum $name { $( $variant => $str ),* });
        impl Default for $name {
            fn default() -> Self { Self::Unknown(String::new()) }
        }
    };
    // Default is a named variant
    ($(#[doc = $doc:expr])* pub enum $name:ident {
        default $default:ident;
        $( $variant:ident => $str:expr ),* $(,)?
    }) => {
        string_enum!(@impl $(#[doc = $doc])* pub enum $name { $( $variant => $str ),* });
        impl Default for $name {
            fn default() -> Self { Self::$default }
        }
    };
    // Internal: generates the enum, as_str, Serialize, Deserialize, Display
    (@impl $(#[doc = $doc:expr])* pub enum $name:ident {
        $( $variant:ident => $str:expr ),*
    }) => {
        $(#[doc = $doc])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $name {
            $( $variant, )*
            Unknown(String),
        }

        impl $name {
            pub fn as_str(&self) -> &str {
                match self {
                    $( Self::$variant => $str, )*
                    Self::Unknown(s) => s,
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let s = String::deserialize(deserializer)?;
                Ok(match s.as_str() {
                    $( $str => Self::$variant, )*
                    _ => Self::Unknown(s),
                })
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

// =============================================================================
// Weapon property enums
// =============================================================================

string_enum! {
    /// Weapon trigger types - how the weapon fires
    pub enum Trigger {
        default Unknown;
        Active => "Active",
        Auto => "Auto",
        AutoBurst => "Auto Burst",
        Burst => "Burst",
        Charge => "Charge",
        Duplex => "Duplex",
        Held => "Held",
        Melee => "Melee",
        Semi => "Semi",
    }
}

string_enum! {
    /// Weapon noise level - affects enemy alertness
    pub enum Noise {
        default Unknown;
        Alarming => "Alarming",
        Silent => "Silent",
    }
}

string_enum! {
    /// Item/drop rarity tiers
    pub enum Rarity {
        default Unknown;
        Common => "Common",
        Uncommon => "Uncommon",
        Rare => "Rare",
        Legendary => "Legendary",
    }
}

string_enum! {
    /// Mod polarity types - determines mod capacity cost
    pub enum Polarity {
        default Unknown;
        Aura => "aura",
        Madurai => "madurai",
        Naramon => "naramon",
        Penjaga => "penjaga",
        Umbra => "umbra",
        Unairu => "unairu",
        Universal => "universal",
        Vazarin => "vazarin",
        Zenurik => "zenurik",
        Any => "any",
    }
}

string_enum! {
    /// Projectile type for ranged weapons
    pub enum Projectile {
        default Unknown;
        Discharge => "Discharge",
        Hitscan => "Hitscan",
        ProjectileType => "Projectile",
        Thrown => "Thrown",
    }
}

string_enum! {
    /// Riven disposition - affects Riven mod stat ranges (1-5)
    pub enum Disposition {
        default Unknown;
        One => "1",
        Two => "2",
        Three => "3",
        Four => "4",
        Five => "5",
    }
}

impl Disposition {
    /// Convert disposition to numeric value (1-5), returns 0 for Unknown
    pub fn as_u8(&self) -> u8 {
        match self {
            Disposition::One => 1,
            Disposition::Two => 2,
            Disposition::Three => 3,
            Disposition::Four => 4,
            Disposition::Five => 5,
            Disposition::Unknown(_) => 0,
        }
    }
}

// =============================================================================
// Item type enums (replacing type_field: String)
// =============================================================================

string_enum! {
    /// Warframe item type
    pub enum WarframeType {
        default Warframe;
        Warframe => "Warframe",
    }
}

string_enum! {
    /// Arcane enhancement type classification
    pub enum ArcaneType {
        default Unknown;
        WarframeArcane => "Warframe Arcane",
        OperatorArcane => "Operator Arcane",
        SecondaryArcane => "Secondary Arcane",
        AmpArcane => "Amp Arcane",
        PrimaryArcane => "Primary Arcane",
        MeleeArcane => "Melee Arcane",
        Arcane => "Arcane",
        KitgunArcane => "Kitgun Arcane",
        ZawArcane => "Zaw Arcane",
        BowArcane => "Bow Arcane",
        ShotgunArcane => "Shotgun Arcane",
    }
}

string_enum! {
    /// Gear item type classification
    pub enum GearType {
        default Gear;
        Gear => "Gear",
        Fish => "Fish",
        FishBait => "Fish Bait",
        Specter => "Specter",
        Key => "Key",
    }
}

string_enum! {
    /// Resource type classification
    pub enum ResourceType {
        default Resource;
        Resource => "Resource",
        Gem => "Gem",
        Plant => "Plant",
    }
}

string_enum! {
    /// Primary weapon type classification
    pub enum PrimaryType {
        default Rifle;
        Bow => "Bow",
        Launcher => "Launcher",
        Pistol => "Pistol",
        Rifle => "Rifle",
        Shotgun => "Shotgun",
        Sniper => "Sniper",
    }
}

string_enum! {
    /// Secondary weapon type classification
    pub enum SecondaryType {
        default Pistol;
        DualPistols => "Dual Pistols",
        Pistol => "Pistol",
        Throwing => "Throwing",
    }
}

string_enum! {
    /// Melee weapon type classification
    pub enum MeleeType {
        default Melee;
        Melee => "Melee",
        Rifle => "Rifle",
        ZawComponent => "Zaw Component",
    }
}

string_enum! {
    /// Railjack weapon types
    pub enum RailjackType {
        default RailjackTurret;
        RailjackTurret => "Railjack Turret",
        Shotgun => "Shotgun",
    }
}

string_enum! {
    /// Arch-Gun item type
    pub enum ArchGunType {
        default ArchGun;
        ArchGun => "Arch-Gun",
    }
}

string_enum! {
    /// Arch-Melee item type
    pub enum ArchMeleeType {
        default ArchMelee;
        ArchMelee => "Arch-Melee",
    }
}

string_enum! {
    /// Archwing item type
    pub enum ArchwingType {
        default Archwing;
        Archwing => "Archwing",
    }
}

string_enum! {
    /// Sentinel item type
    pub enum SentinelType {
        default Sentinel;
        Sentinel => "Sentinel",
    }
}

string_enum! {
    /// Sentinel weapon item type
    pub enum SentinelWeaponType {
        default CompanionWeapon;
        CompanionWeapon => "Companion Weapon",
    }
}

string_enum! {
    /// Mod item type classification
    pub enum ModType {
        default Unknown;
        ArchGunMod => "Arch-Gun Mod",
        ArchGunRivenMod => "Arch-Gun Riven Mod",
        ArchMeleeMod => "Arch-Melee Mod",
        ArchwingMod => "Archwing Mod",
        CompanionMod => "Companion Mod",
        CompanionWeaponRivenMod => "Companion Weapon Riven Mod",
        FocusWay => "Focus Way",
        KDriveMod => "K-Drive Mod",
        KitgunRivenMod => "Kitgun Riven Mod",
        MeleeMod => "Melee Mod",
        MeleeRivenMod => "Melee Riven Mod",
        ModSetMod => "Mod Set Mod",
        NecramechMod => "Necramech Mod",
        ParazonMod => "Parazon Mod",
        PeculiarMod => "Peculiar Mod",
        PistolRivenMod => "Pistol Riven Mod",
        PlexusMod => "Plexus Mod",
        PostureMod => "Posture Mod",
        PrimaryMod => "Primary Mod",
        RailjackMod => "Railjack Mod",
        RifleRivenMod => "Rifle Riven Mod",
        SecondaryMod => "Secondary Mod",
        ShotgunMod => "Shotgun Mod",
        ShotgunRivenMod => "Shotgun Riven Mod",
        StanceMod => "Stance Mod",
        TektolystArtifactMod => "Tektolyst Artifact Mod",
        TransmutationMod => "Transmutation Mod",
        WarframeMod => "Warframe Mod",
        ZawRivenMod => "Zaw Riven Mod",
    }
}

string_enum! {
    /// Relic item type
    pub enum RelicType {
        default Relic;
        Relic => "Relic",
    }
}

string_enum! {
    /// Fish item type
    pub enum FishType {
        default Fish;
        Fish => "Fish",
    }
}

string_enum! {
    /// Glyph item type
    pub enum GlyphType {
        default Glyph;
        Glyph => "Glyph",
    }
}

string_enum! {
    /// Sigil item type
    pub enum SigilType {
        default Sigil;
        Sigil => "Sigil",
    }
}

string_enum! {
    /// Node item type
    pub enum NodeType {
        default Node;
        Node => "Node",
    }
}

string_enum! {
    /// Quest item type
    pub enum QuestType {
        default Key;
        Key => "Key",
    }
}

string_enum! {
    /// Skin/cosmetic item type classification
    pub enum SkinType {
        default Skin;
        ArcadeMinigameUnlock => "Arcade Minigame Unlock",
        ColorPalette => "Color Palette",
        Emotes => "Emotes",
        FurColor => "Fur Color",
        FurPattern => "Fur Pattern",
        Glyph => "Glyph",
        Misc => "Misc",
        NotePacks => "Note Packs",
        Resource => "Resource",
        ShipDecoration => "Ship Decoration",
        Skin => "Skin",
        Skins => "Skins",
        Syandana => "Syandana",
        ThemeBackground => "Theme Background",
        ThemeSound => "Theme Sound",
        Themes => "Themes",
    }
}

string_enum! {
    /// Enemy faction/type classification
    pub enum EnemyType {
        default Unknown;
        Corpus => "Corpus",
        Grineer => "Grineer",
        Infestation => "Infestation",
        Melee => "Melee",
        Neutral => "Neutral",
        Orbiter => "Orbiter",
        Orokin => "Orokin",
        Predator => "Predator",
        Prey => "Prey",
        Rifle => "Rifle",
        Sentient => "Sentient",
        Shotgun => "Shotgun",
        Stalker => "Stalker",
        Tenno => "Tenno",
        Warframe => "Warframe",
    }
}

string_enum! {
    /// Pet item type classification
    pub enum PetType {
        default Unknown;
        PetParts => "Pet Parts",
        PetResource => "Pet Resource",
        Pets => "Pets",
        Warframe => "Warframe",
    }
}

string_enum! {
    /// Miscellaneous item type classification
    pub enum MiscType {
        default Misc;
        Alloy => "Alloy",
        Amp => "Amp",
        AyatanSculpture => "Ayatan Sculpture",
        AyatanStar => "Ayatan Star",
        Boosters => "Boosters",
        Captura => "Captura",
        ConservationPrey => "Conservation Prey",
        ConservationTag => "Conservation Tag",
        Currency => "Currency",
        CutGem => "Cut Gem",
        EidolonShard => "Eidolon Shard",
        EquipmentAdapter => "Equipment Adapter",
        ExaltedWeapon => "Exalted Weapon",
        Extractor => "Extractor",
        FishBait => "Fish Bait",
        FishPart => "Fish Part",
        FocusLens => "Focus Lens",
        KDriveComponent => "K-Drive Component",
        Key => "Key",
        KitgunComponent => "Kitgun Component",
        KitgunRivenMod => "Kitgun Riven Mod",
        Medallion => "Medallion",
        MeleeRivenMod => "Melee Riven Mod",
        Misc => "Misc",
        NightwaveChallenge => "Nightwave Challenge",
        Orbiter => "Orbiter",
        PetCollar => "Pet Collar",
        PetResource => "Pet Resource",
        Pistol => "Pistol",
        PistolRivenMod => "Pistol Riven Mod",
        Resource => "Resource",
        Rifle => "Rifle",
        RifleRivenMod => "Rifle Riven Mod",
        ShipSegment => "Ship Segment",
        ShotgunRivenMod => "Shotgun Riven Mod",
        Simulacrum => "Simulacrum",
        Skin => "Skin",
        ZawRivenMod => "Zaw Riven Mod",
    }
}

string_enum! {
    /// Enemy resistance type classification
    pub enum ResistanceType {
        default Unknown;
        AlloyArmor => "Alloy Armor",
        ClonedFlesh => "Cloned Flesh",
        FeriteArmor => "Ferrite Armor",
        Flesh => "Flesh",
        Fossilized => "Fossilized",
        Infested => "Infested",
        InfestedFlesh => "Infested Flesh",
        InfestedSinew => "Infested Sinew",
        Machinery => "Machinery",
        NoResistance => "None",
        ProtoShield => "Proto Shield",
        Robotic => "Robotic",
        Shield => "Shield",
    }
}

string_enum! {
    /// Component item type
    pub enum ComponentType {
        default Resource;
        Resource => "Resource",
    }
}

// =============================================================================
// Product category enums
// =============================================================================

string_enum! {
    /// Primary weapon product category
    pub enum PrimaryProductCategory {
        default LongGuns;
        LongGuns => "LongGuns",
        OperatorAmps => "OperatorAmps",
    }
}

string_enum! {
    /// Melee weapon product category
    pub enum MeleeProductCategory {
        default Melee;
        Melee => "Melee",
        Pistols => "Pistols",
    }
}

string_enum! {
    /// Warframe sex/gender
    pub enum Sex {
        default Unknown;
        Female => "Female",
        Male => "Male",
        NonBinary => "Non-binary",
    }
}

string_enum! {
    /// Secondary weapon product category
    pub enum SecondaryProductCategory {
        default Pistols;
        Pistols => "Pistols",
    }
}

string_enum! {
    /// Archwing product category
    pub enum ArchwingProductCategory {
        default SpaceSuits;
        SpaceSuits => "SpaceSuits",
    }
}

string_enum! {
    /// Sentinel product category
    pub enum SentinelProductCategory {
        default Sentinels;
        Sentinels => "Sentinels",
    }
}

string_enum! {
    /// Sentinel weapon product category
    pub enum SentinelWeaponProductCategory {
        default SentinelWeapons;
        SentinelWeapons => "SentinelWeapons",
    }
}

string_enum! {
    /// Arch-Gun product category
    pub enum ArchGunProductCategory {
        default SpaceGuns;
        SpaceGuns => "SpaceGuns",
    }
}

string_enum! {
    /// Arch-Melee product category
    pub enum ArchMeleeProductCategory {
        default SpaceMelee;
        SpaceMelee => "SpaceMelee",
    }
}

// =============================================================================
// Equipment slot (numeric, not string-based)
// =============================================================================

/// Equipment slot classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, Default)]
#[repr(i64)]
pub enum Slot {
    /// Secondary weapons
    Secondary = 0,
    /// Primary, Arch-Gun, SentinelWeapons (ranged)
    #[default]
    Primary = 1,
    /// Grimoire anomaly
    SpecialSecondary = 2,
    /// Melee, Arch-Melee
    Melee = 5,
}

// =============================================================================
// Computed enums (not directly deserialized from JSON)
// =============================================================================

/// Vault status for Prime items - represents the lifecycle state.
///
/// This is a computed enum derived from the combination of `is_prime`,
/// `vaulted`, `vault_date`, and `estimated_vault_date` fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum VaultStatus {
    /// Not a Prime item (is_prime = false)
    #[default]
    NotPrime,
    /// Prime item currently available (vaulted = false, no estimated date)
    Active,
    /// Prime item with predicted vault date (vaulted = false, has estimated date)
    EstimatedVault { estimated_date: String },
    /// Prime item currently vaulted (vaulted = true)
    Vaulted { date: String },
}

impl VaultStatus {
    /// Compute vault status from individual fields.
    pub fn from_fields(
        is_prime: bool,
        vaulted: Option<bool>,
        vault_date: Option<&str>,
        estimated_vault_date: Option<&str>,
    ) -> Self {
        if !is_prime {
            return VaultStatus::NotPrime;
        }

        match (vaulted, vault_date, estimated_vault_date) {
            (Some(true), Some(date), _) => VaultStatus::Vaulted {
                date: date.to_string(),
            },
            (Some(true), None, _) => VaultStatus::Vaulted {
                date: String::new(),
            },
            (_, _, Some(est_date)) if !vaulted.unwrap_or(false) => VaultStatus::EstimatedVault {
                estimated_date: est_date.to_string(),
            },
            _ => VaultStatus::Active,
        }
    }

    pub fn is_prime(&self) -> bool {
        !matches!(self, VaultStatus::NotPrime)
    }

    pub fn is_vaulted(&self) -> bool {
        matches!(self, VaultStatus::Vaulted { .. })
    }

    pub fn is_accessible(&self) -> bool {
        !self.is_vaulted()
    }

    pub fn vault_date(&self) -> Option<&str> {
        match self {
            VaultStatus::Vaulted { date } if !date.is_empty() => Some(date),
            _ => None,
        }
    }

    pub fn estimated_vault_date(&self) -> Option<&str> {
        match self {
            VaultStatus::EstimatedVault { estimated_date } => Some(estimated_date),
            _ => None,
        }
    }
}

/// Mod category classification.
///
/// This is a computed enum derived from mod field presence.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum ModCategory {
    /// Riven mod with unveiling challenges
    Riven,
    /// Member of a mod set (references a set definition)
    SetMember {
        mod_set: String,
    },
    /// Mod set definition (describes set bonuses)
    SetDefinition {
        num_upgrades_in_set: i64,
    },
    /// Regular mod with level-based stats
    #[default]
    Regular,
}

impl ModCategory {
    pub fn is_riven(&self) -> bool {
        matches!(self, ModCategory::Riven)
    }

    pub fn is_set(&self) -> bool {
        matches!(
            self,
            ModCategory::SetMember { .. } | ModCategory::SetDefinition { .. }
        )
    }

    pub fn is_set_member(&self) -> bool {
        matches!(self, ModCategory::SetMember { .. })
    }

    pub fn is_set_definition(&self) -> bool {
        matches!(self, ModCategory::SetDefinition { .. })
    }

    pub fn is_regular(&self) -> bool {
        matches!(self, ModCategory::Regular)
    }

    pub fn mod_set(&self) -> Option<&str> {
        match self {
            ModCategory::SetMember { mod_set } => Some(mod_set),
            _ => None,
        }
    }

    pub fn num_upgrades_in_set(&self) -> Option<i64> {
        match self {
            ModCategory::SetDefinition {
                num_upgrades_in_set,
            } => Some(*num_upgrades_in_set),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_deserialize() {
        let trigger: Trigger = serde_json::from_str(r#""Auto Burst""#).unwrap();
        assert_eq!(trigger, Trigger::AutoBurst);

        let trigger: Trigger = serde_json::from_str(r#""Semi""#).unwrap();
        assert_eq!(trigger, Trigger::Semi);

        // Unknown variant captures value
        let trigger: Trigger = serde_json::from_str(r#""NewTriggerType""#).unwrap();
        assert_eq!(trigger, Trigger::Unknown("NewTriggerType".to_string()));
    }

    #[test]
    fn test_trigger_serialize_unknown() {
        let trigger = Trigger::Unknown("NewTriggerType".to_string());
        let json = serde_json::to_string(&trigger).unwrap();
        assert_eq!(json, r#""NewTriggerType""#);
    }

    #[test]
    fn test_polarity_deserialize() {
        let polarity: Polarity = serde_json::from_str(r#""madurai""#).unwrap();
        assert_eq!(polarity, Polarity::Madurai);

        let polarity: Polarity = serde_json::from_str(r#""vazarin""#).unwrap();
        assert_eq!(polarity, Polarity::Vazarin);
    }

    #[test]
    fn test_polarity_serialize_unknown() {
        let polarity = Polarity::Unknown("newpolarity".to_string());
        let json = serde_json::to_string(&polarity).unwrap();
        assert_eq!(json, r#""newpolarity""#);

        let roundtrip: Polarity = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtrip, Polarity::Unknown("newpolarity".to_string()));
    }

    #[test]
    fn test_rarity_deserialize() {
        let rarity: Rarity = serde_json::from_str(r#""Legendary""#).unwrap();
        assert_eq!(rarity, Rarity::Legendary);

        let rarity: Rarity = serde_json::from_str(r#""Common""#).unwrap();
        assert_eq!(rarity, Rarity::Common);
    }

    #[test]
    fn test_rarity_serialize_unknown() {
        let rarity = Rarity::Unknown("Mythic".to_string());
        let json = serde_json::to_string(&rarity).unwrap();
        assert_eq!(json, r#""Mythic""#);

        let roundtrip: Rarity = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtrip, Rarity::Unknown("Mythic".to_string()));
    }

    #[test]
    fn test_disposition_as_u8() {
        assert_eq!(Disposition::Three.as_u8(), 3);
        assert_eq!(Disposition::Unknown("?".to_string()).as_u8(), 0);
    }

    #[test]
    fn test_vault_status_not_prime() {
        let status = VaultStatus::from_fields(false, None, None, None);
        assert_eq!(status, VaultStatus::NotPrime);
        assert!(!status.is_prime());
        assert!(!status.is_vaulted());
        assert!(status.is_accessible());
    }

    #[test]
    fn test_vault_status_active_prime() {
        let status = VaultStatus::from_fields(true, Some(false), None, None);
        assert_eq!(status, VaultStatus::Active);
        assert!(status.is_prime());
        assert!(!status.is_vaulted());
        assert!(status.is_accessible());
    }

    #[test]
    fn test_vault_status_vaulted_prime() {
        let status = VaultStatus::from_fields(true, Some(true), Some("2021-09-08"), None);
        assert!(matches!(status, VaultStatus::Vaulted { .. }));
        assert!(status.is_prime());
        assert!(status.is_vaulted());
        assert!(!status.is_accessible());
        assert_eq!(status.vault_date(), Some("2021-09-08"));
    }

    #[test]
    fn test_vault_status_estimated_vault() {
        let status = VaultStatus::from_fields(true, Some(false), None, Some("2023-03-14"));
        assert!(matches!(status, VaultStatus::EstimatedVault { .. }));
        assert!(status.is_prime());
        assert!(!status.is_vaulted());
        assert!(status.is_accessible());
        assert_eq!(status.estimated_vault_date(), Some("2023-03-14"));
    }

    #[test]
    fn test_mod_category_regular() {
        let cat = ModCategory::Regular;
        assert!(cat.is_regular());
        assert!(!cat.is_riven());
        assert!(!cat.is_set());
        assert!(cat.mod_set().is_none());
        assert!(cat.num_upgrades_in_set().is_none());
    }

    #[test]
    fn test_mod_category_riven() {
        let cat = ModCategory::Riven;
        assert!(cat.is_riven());
        assert!(!cat.is_regular());
        assert!(!cat.is_set());
    }

    #[test]
    fn test_mod_category_set_member() {
        let cat = ModCategory::SetMember {
            mod_set: "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod".to_string(),
        };
        assert!(cat.is_set());
        assert!(cat.is_set_member());
        assert!(!cat.is_set_definition());
        assert!(!cat.is_regular());
        assert_eq!(
            cat.mod_set(),
            Some("/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod")
        );
    }

    #[test]
    fn test_mod_category_set_definition() {
        let cat = ModCategory::SetDefinition {
            num_upgrades_in_set: 3,
        };
        assert!(cat.is_set());
        assert!(cat.is_set_definition());
        assert!(!cat.is_set_member());
        assert!(!cat.is_regular());
        assert_eq!(cat.num_upgrades_in_set(), Some(3));
    }

    // Unknown(String) serialize/deserialize roundtrip tests for all type enums

    macro_rules! test_unknown_roundtrip {
        ($name:ident, $enum_type:ty, $unknown_val:expr) => {
            #[test]
            fn $name() {
                let val = <$enum_type>::Unknown($unknown_val.to_string());
                let json = serde_json::to_string(&val).unwrap();
                assert_eq!(json, format!("\"{}\"", $unknown_val));

                let roundtrip: $enum_type = serde_json::from_str(&json).unwrap();
                assert_eq!(roundtrip, <$enum_type>::Unknown($unknown_val.to_string()));
            }
        };
    }

    test_unknown_roundtrip!(test_warframe_type_unknown, WarframeType, "NewWarframeType");
    test_unknown_roundtrip!(test_arcane_type_unknown, ArcaneType, "New Arcane");
    test_unknown_roundtrip!(test_gear_type_unknown, GearType, "New Gear");
    test_unknown_roundtrip!(test_resource_type_unknown, ResourceType, "New Resource");
    test_unknown_roundtrip!(test_primary_type_unknown, PrimaryType, "New Primary");
    test_unknown_roundtrip!(test_secondary_type_unknown, SecondaryType, "New Secondary");
    test_unknown_roundtrip!(test_melee_type_unknown, MeleeType, "New Melee");
    test_unknown_roundtrip!(test_railjack_type_unknown, RailjackType, "New Railjack");
    test_unknown_roundtrip!(test_arch_gun_type_unknown, ArchGunType, "New Arch-Gun");
    test_unknown_roundtrip!(test_arch_melee_type_unknown, ArchMeleeType, "New Arch-Melee");
    test_unknown_roundtrip!(test_archwing_type_unknown, ArchwingType, "New Archwing");
    test_unknown_roundtrip!(test_sentinel_type_unknown, SentinelType, "New Sentinel");
    test_unknown_roundtrip!(test_sentinel_weapon_type_unknown, SentinelWeaponType, "New Companion");
    test_unknown_roundtrip!(test_mod_type_unknown, ModType, "New Mod Type");
    test_unknown_roundtrip!(test_relic_type_unknown, RelicType, "New Relic");
    test_unknown_roundtrip!(test_fish_type_unknown, FishType, "New Fish");
    test_unknown_roundtrip!(test_glyph_type_unknown, GlyphType, "New Glyph");
    test_unknown_roundtrip!(test_sigil_type_unknown, SigilType, "New Sigil");
    test_unknown_roundtrip!(test_node_type_unknown, NodeType, "New Node");
    test_unknown_roundtrip!(test_quest_type_unknown, QuestType, "New Quest");
    test_unknown_roundtrip!(test_skin_type_unknown, SkinType, "New Skin");
    test_unknown_roundtrip!(test_enemy_type_unknown, EnemyType, "New Enemy");
    test_unknown_roundtrip!(test_pet_type_unknown, PetType, "New Pet");
    test_unknown_roundtrip!(test_misc_type_unknown, MiscType, "New Misc");
    test_unknown_roundtrip!(test_resistance_type_unknown, ResistanceType, "New Resistance");
    test_unknown_roundtrip!(test_component_type_unknown, ComponentType, "New Component");
    test_unknown_roundtrip!(test_noise_unknown, Noise, "New Noise");
    test_unknown_roundtrip!(test_projectile_unknown, Projectile, "New Projectile");
    test_unknown_roundtrip!(test_sex_unknown, Sex, "New Sex");
    test_unknown_roundtrip!(test_disposition_unknown, Disposition, "6");
}

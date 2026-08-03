//! Enums for known finite value sets in item data.
//!
//! These replace String fields where the set of valid values is known and finite.
//! Each enum includes an `Unknown(String)` variant to gracefully handle new values
//! that may be added to the source data in the future, while preserving the original value.

use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{AsRefStr, Display, EnumString};

// =============================================================================
// Weapon property enums
// =============================================================================

/// Weapon trigger types - how the weapon fires
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum Trigger {
    Active,
    Auto,
    #[strum(serialize = "Auto Burst")]
    AutoBurst,
    Burst,
    Charge,
    Duplex,
    Held,
    Melee,
    Semi,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for Trigger {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Weapon noise level - affects enemy alertness
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum Noise {
    Alarming,
    Silent,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for Noise {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Item/drop rarity tiers
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for Rarity {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Mod polarity types - determines mod capacity cost
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum Polarity {
    Aura,
    Madurai,
    Naramon,
    Penjaga,
    Umbra,
    Unairu,
    Universal,
    Vazarin,
    Zenurik,
    Any,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for Polarity {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Projectile type for ranged weapons
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[expect(
    clippy::enum_variant_names,
    reason = "wire value for the projectile-launching variant is literally \"Projectile\""
)]
pub enum Projectile {
    Discharge,
    Hitscan,
    #[strum(serialize = "Projectile")]
    ProjectileType,
    Thrown,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for Projectile {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Riven disposition - affects Riven mod stat ranges (1-5)
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum Disposition {
    #[strum(serialize = "1")]
    One,
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for Disposition {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

impl Disposition {
    /// Convert disposition to numeric value (1-5), returns 0 for Unknown
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Unknown(_) => 0,
        }
    }
}

// =============================================================================
// Item type enums (replacing type_field: String)
// =============================================================================

/// Warframe item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum WarframeType {
    #[default]
    Warframe,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Arcane enhancement type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum ArcaneType {
    #[strum(serialize = "Warframe Arcane")]
    WarframeArcane,
    #[strum(serialize = "Operator Arcane")]
    OperatorArcane,
    #[strum(serialize = "Secondary Arcane")]
    SecondaryArcane,
    #[strum(serialize = "Amp Arcane")]
    AmpArcane,
    #[strum(serialize = "Primary Arcane")]
    PrimaryArcane,
    #[strum(serialize = "Melee Arcane")]
    MeleeArcane,
    Arcane,
    #[strum(serialize = "Kitgun Arcane")]
    KitgunArcane,
    #[strum(serialize = "Zaw Arcane")]
    ZawArcane,
    #[strum(serialize = "Bow Arcane")]
    BowArcane,
    #[strum(serialize = "Shotgun Arcane")]
    ShotgunArcane,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for ArcaneType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Gear item type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum GearType {
    #[default]
    Gear,
    Fish,
    #[strum(serialize = "Fish Bait")]
    FishBait,
    Specter,
    Key,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Resource type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum ResourceType {
    #[default]
    Resource,
    Gem,
    Plant,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Primary weapon type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum PrimaryType {
    Bow,
    Launcher,
    Pistol,
    #[default]
    Rifle,
    Shotgun,
    Sniper,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Secondary weapon type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum SecondaryType {
    #[strum(serialize = "Dual Pistols")]
    DualPistols,
    #[default]
    Pistol,
    Throwing,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Melee weapon type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum MeleeType {
    #[default]
    Melee,
    Rifle,
    #[strum(serialize = "Zaw Component")]
    ZawComponent,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Railjack weapon types
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum RailjackType {
    #[default]
    #[strum(serialize = "Railjack Turret")]
    RailjackTurret,
    Shotgun,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Arch-Gun item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum ArchGunType {
    #[default]
    #[strum(serialize = "Arch-Gun")]
    ArchGun,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Arch-Melee item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum ArchMeleeType {
    #[default]
    #[strum(serialize = "Arch-Melee")]
    ArchMelee,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Archwing item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum ArchwingType {
    #[default]
    Archwing,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Sentinel item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum SentinelType {
    #[default]
    Sentinel,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Sentinel weapon item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum SentinelWeaponType {
    #[default]
    #[strum(serialize = "Companion Weapon")]
    CompanionWeapon,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Mod item type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum ModType {
    #[strum(serialize = "Arch-Gun Mod")]
    ArchGunMod,
    #[strum(serialize = "Arch-Gun Riven Mod")]
    ArchGunRivenMod,
    #[strum(serialize = "Arch-Melee Mod")]
    ArchMeleeMod,
    #[strum(serialize = "Archwing Mod")]
    ArchwingMod,
    #[strum(serialize = "Companion Mod")]
    CompanionMod,
    #[strum(serialize = "Companion Weapon Riven Mod")]
    CompanionWeaponRivenMod,
    #[strum(serialize = "Focus Way")]
    FocusWay,
    #[strum(serialize = "K-Drive Mod")]
    KDriveMod,
    #[strum(serialize = "Kitgun Riven Mod")]
    KitgunRivenMod,
    #[strum(serialize = "Melee Mod")]
    MeleeMod,
    #[strum(serialize = "Melee Riven Mod")]
    MeleeRivenMod,
    #[strum(serialize = "Mod Set Mod")]
    ModSetMod,
    #[strum(serialize = "Necramech Mod")]
    NecramechMod,
    #[strum(serialize = "Parazon Mod")]
    ParazonMod,
    #[strum(serialize = "Peculiar Mod")]
    PeculiarMod,
    #[strum(serialize = "Pistol Riven Mod")]
    PistolRivenMod,
    #[strum(serialize = "Plexus Mod")]
    PlexusMod,
    #[strum(serialize = "Posture Mod")]
    PostureMod,
    #[strum(serialize = "Primary Mod")]
    PrimaryMod,
    #[strum(serialize = "Railjack Mod")]
    RailjackMod,
    #[strum(serialize = "Rifle Riven Mod")]
    RifleRivenMod,
    #[strum(serialize = "Secondary Mod")]
    SecondaryMod,
    #[strum(serialize = "Shotgun Mod")]
    ShotgunMod,
    #[strum(serialize = "Shotgun Riven Mod")]
    ShotgunRivenMod,
    #[strum(serialize = "Stance Mod")]
    StanceMod,
    #[strum(serialize = "Tektolyst Artifact Mod")]
    TektolystArtifactMod,
    #[strum(serialize = "Transmutation Mod")]
    TransmutationMod,
    #[strum(serialize = "Warframe Mod")]
    WarframeMod,
    #[strum(serialize = "Zaw Riven Mod")]
    ZawRivenMod,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for ModType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Relic item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum RelicType {
    #[default]
    Relic,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Fish item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum FishType {
    #[default]
    Fish,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Glyph item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum GlyphType {
    #[default]
    Glyph,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Sigil item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum SigilType {
    #[default]
    Sigil,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Node item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum NodeType {
    #[default]
    Node,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Quest item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum QuestType {
    #[default]
    Key,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Skin/cosmetic item type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum SkinType {
    #[strum(serialize = "Arcade Minigame Unlock")]
    ArcadeMinigameUnlock,
    #[strum(serialize = "Color Palette")]
    ColorPalette,
    Emotes,
    #[strum(serialize = "Fur Color")]
    FurColor,
    #[strum(serialize = "Fur Pattern")]
    FurPattern,
    Glyph,
    Misc,
    #[strum(serialize = "Note Packs")]
    NotePacks,
    Resource,
    #[strum(serialize = "Ship Decoration")]
    ShipDecoration,
    #[default]
    Skin,
    Skins,
    Syandana,
    #[strum(serialize = "Theme Background")]
    ThemeBackground,
    #[strum(serialize = "Theme Sound")]
    ThemeSound,
    Themes,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Enemy faction/type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum EnemyType {
    Corpus,
    Grineer,
    Infestation,
    Melee,
    Neutral,
    Orbiter,
    Orokin,
    Predator,
    Prey,
    Rifle,
    Sentient,
    Shotgun,
    Stalker,
    Tenno,
    Warframe,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for EnemyType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Pet item type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum PetType {
    #[strum(serialize = "Pet Parts")]
    PetParts,
    #[strum(serialize = "Pet Resource")]
    PetResource,
    Pets,
    Warframe,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for PetType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Miscellaneous item type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum MiscType {
    Alloy,
    Amp,
    #[strum(serialize = "Ayatan Sculpture")]
    AyatanSculpture,
    #[strum(serialize = "Ayatan Star")]
    AyatanStar,
    Boosters,
    Captura,
    #[strum(serialize = "Conservation Prey")]
    ConservationPrey,
    #[strum(serialize = "Conservation Tag")]
    ConservationTag,
    Currency,
    #[strum(serialize = "Cut Gem")]
    CutGem,
    #[strum(serialize = "Eidolon Shard")]
    EidolonShard,
    #[strum(serialize = "Equipment Adapter")]
    EquipmentAdapter,
    #[strum(serialize = "Exalted Weapon")]
    ExaltedWeapon,
    Extractor,
    #[strum(serialize = "Fish Bait")]
    FishBait,
    #[strum(serialize = "Fish Part")]
    FishPart,
    #[strum(serialize = "Focus Lens")]
    FocusLens,
    #[strum(serialize = "K-Drive Component")]
    KDriveComponent,
    Key,
    #[strum(serialize = "Kitgun Component")]
    KitgunComponent,
    #[strum(serialize = "Kitgun Riven Mod")]
    KitgunRivenMod,
    Medallion,
    #[strum(serialize = "Melee Riven Mod")]
    MeleeRivenMod,
    #[default]
    Misc,
    #[strum(serialize = "Nightwave Challenge")]
    NightwaveChallenge,
    Orbiter,
    #[strum(serialize = "Pet Collar")]
    PetCollar,
    #[strum(serialize = "Pet Resource")]
    PetResource,
    Pistol,
    #[strum(serialize = "Pistol Riven Mod")]
    PistolRivenMod,
    Resource,
    Rifle,
    #[strum(serialize = "Rifle Riven Mod")]
    RifleRivenMod,
    #[strum(serialize = "Ship Segment")]
    ShipSegment,
    #[strum(serialize = "Shotgun Riven Mod")]
    ShotgunRivenMod,
    Simulacrum,
    Skin,
    #[strum(serialize = "Zaw Riven Mod")]
    ZawRivenMod,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Enemy resistance type classification
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum ResistanceType {
    #[strum(serialize = "Alloy Armor")]
    AlloyArmor,
    #[strum(serialize = "Cloned Flesh")]
    ClonedFlesh,
    #[strum(serialize = "Ferrite Armor")]
    FeriteArmor,
    Flesh,
    Fossilized,
    Infested,
    #[strum(serialize = "Infested Flesh")]
    InfestedFlesh,
    #[strum(serialize = "Infested Sinew")]
    InfestedSinew,
    Machinery,
    #[strum(serialize = "None")]
    NoResistance,
    #[strum(serialize = "Proto Shield")]
    ProtoShield,
    Robotic,
    Shield,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for ResistanceType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Component item type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum ComponentType {
    #[default]
    Resource,
    #[strum(default, transparent)]
    Unknown(String),
}

// =============================================================================
// Product category enums
// =============================================================================

/// Primary weapon product category
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum PrimaryProductCategory {
    #[default]
    LongGuns,
    OperatorAmps,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Melee weapon product category
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum MeleeProductCategory {
    #[default]
    Melee,
    Pistols,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Warframe sex/gender
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum Sex {
    Female,
    Male,
    #[strum(serialize = "Non-binary")]
    NonBinary,
    #[strum(default, transparent)]
    Unknown(String),
}

impl Default for Sex {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Secondary weapon product category
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum SecondaryProductCategory {
    #[default]
    Pistols,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Archwing product category
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum ArchwingProductCategory {
    #[default]
    SpaceSuits,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Sentinel product category
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum SentinelProductCategory {
    #[default]
    Sentinels,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Sentinel weapon product category
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum SentinelWeaponProductCategory {
    #[default]
    SentinelWeapons,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Arch-Gun product category
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum ArchGunProductCategory {
    #[default]
    SpaceGuns,
    #[strum(default, transparent)]
    Unknown(String),
}

/// Arch-Melee product category
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    Display,
    EnumString,
    AsRefStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum ArchMeleeProductCategory {
    #[default]
    SpaceMelee,
    #[strum(default, transparent)]
    Unknown(String),
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, derive_more::IsVariant)]
pub enum VaultStatus {
    /// Not a Prime item (is_prime = false)
    #[default]
    NotPrime,
    /// Prime item currently available (vaulted = false, no estimated date)
    Active,
    /// Prime item with predicted vault date (vaulted = false, has estimated date)
    EstimatedVault { estimated_date: String },
    /// Prime item currently vaulted (vaulted = true); the vault date is not
    /// always known.
    Vaulted { date: Option<String> },
}

impl VaultStatus {
    #[must_use]
    pub const fn is_prime(&self) -> bool {
        !self.is_not_prime()
    }

    #[must_use]
    pub const fn is_accessible(&self) -> bool {
        !self.is_vaulted()
    }

    #[must_use]
    pub fn vault_date(&self) -> Option<&str> {
        match self {
            Self::Vaulted { date } => date.as_deref(),
            _ => None,
        }
    }

    #[must_use]
    pub fn estimated_vault_date(&self) -> Option<&str> {
        match self {
            Self::EstimatedVault { estimated_date } => Some(estimated_date),
            _ => None,
        }
    }
}

/// Mod category classification.
///
/// This is a computed enum derived from mod field presence.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, derive_more::IsVariant)]
pub enum ModCategory {
    /// Riven mod with unveiling challenges
    Riven,
    /// Member of a mod set (references a set definition)
    SetMember { mod_set: String },
    /// Mod set definition (describes set bonuses)
    SetDefinition { num_upgrades_in_set: i64 },
    /// Regular mod with level-based stats
    #[default]
    Regular,
}

impl ModCategory {
    #[must_use]
    pub const fn is_set(&self) -> bool {
        self.is_set_member() || self.is_set_definition()
    }

    #[must_use]
    pub fn mod_set(&self) -> Option<&str> {
        match self {
            Self::SetMember { mod_set } => Some(mod_set),
            _ => None,
        }
    }

    #[must_use]
    pub const fn num_upgrades_in_set(&self) -> Option<i64> {
        match self {
            Self::SetDefinition {
                num_upgrades_in_set,
            } => Some(*num_upgrades_in_set),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_trigger_deserialize() {
        let trigger: Trigger = serde_json::from_str(r#""Auto Burst""#).unwrap();
        assert_eq!(trigger, Trigger::AutoBurst);

        let trigger: Trigger = serde_json::from_str(r#""Semi""#).unwrap();
        assert_eq!(trigger, Trigger::Semi);

        // Unknown variant captures value
        let trigger: Trigger = serde_json::from_str(r#""NewTriggerType""#).unwrap();
        assert_eq!(trigger, Trigger::Unknown("NewTriggerType".to_owned()));
    }

    #[test]
    fn test_trigger_serialize_unknown() {
        let trigger = Trigger::Unknown("NewTriggerType".to_owned());
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
        let polarity = Polarity::Unknown("newpolarity".to_owned());
        let json = serde_json::to_string(&polarity).unwrap();
        assert_eq!(json, r#""newpolarity""#);

        let roundtrip: Polarity = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtrip, Polarity::Unknown("newpolarity".to_owned()));
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
        let rarity = Rarity::Unknown("Mythic".to_owned());
        let json = serde_json::to_string(&rarity).unwrap();
        assert_eq!(json, r#""Mythic""#);

        let roundtrip: Rarity = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtrip, Rarity::Unknown("Mythic".to_owned()));
    }

    #[test]
    fn test_disposition_as_u8() {
        assert_eq!(Disposition::Three.as_u8(), 3);
        assert_eq!(Disposition::Unknown("?".to_owned()).as_u8(), 0);
    }

    #[test]
    fn test_vault_status_not_prime() {
        let status = VaultStatus::NotPrime;
        assert!(!status.is_prime());
        assert!(!status.is_vaulted());
        assert!(status.is_accessible());
    }

    #[test]
    fn test_vault_status_active_prime() {
        let status = VaultStatus::Active;
        assert!(status.is_prime());
        assert!(!status.is_vaulted());
        assert!(status.is_accessible());
    }

    #[test]
    fn test_vault_status_vaulted_prime() {
        let status = VaultStatus::Vaulted {
            date: Some("2021-09-08".to_owned()),
        };
        assert!(status.is_prime());
        assert!(status.is_vaulted());
        assert!(!status.is_accessible());
        assert_eq!(status.vault_date(), Some("2021-09-08"));

        let undated = VaultStatus::Vaulted { date: None };
        assert!(undated.is_vaulted());
        assert_eq!(undated.vault_date(), None);
    }

    #[test]
    fn test_vault_status_estimated_vault() {
        let status = VaultStatus::EstimatedVault {
            estimated_date: "2023-03-14".to_owned(),
        };
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
            mod_set: "/Lotus/Upgrades/Mods/Sets/Amar/AmarSetMod".to_owned(),
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

    test_unknown_roundtrip!(test_warframe_type_unknown, WarframeType, "NewWarframeType");
    test_unknown_roundtrip!(test_arcane_type_unknown, ArcaneType, "New Arcane");
    test_unknown_roundtrip!(test_gear_type_unknown, GearType, "New Gear");
    test_unknown_roundtrip!(test_resource_type_unknown, ResourceType, "New Resource");
    test_unknown_roundtrip!(test_primary_type_unknown, PrimaryType, "New Primary");
    test_unknown_roundtrip!(test_secondary_type_unknown, SecondaryType, "New Secondary");
    test_unknown_roundtrip!(test_melee_type_unknown, MeleeType, "New Melee");
    test_unknown_roundtrip!(test_railjack_type_unknown, RailjackType, "New Railjack");
    test_unknown_roundtrip!(test_arch_gun_type_unknown, ArchGunType, "New Arch-Gun");
    test_unknown_roundtrip!(
        test_arch_melee_type_unknown,
        ArchMeleeType,
        "New Arch-Melee"
    );
    test_unknown_roundtrip!(test_archwing_type_unknown, ArchwingType, "New Archwing");
    test_unknown_roundtrip!(test_sentinel_type_unknown, SentinelType, "New Sentinel");
    test_unknown_roundtrip!(
        test_sentinel_weapon_type_unknown,
        SentinelWeaponType,
        "New Companion"
    );
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
    test_unknown_roundtrip!(
        test_resistance_type_unknown,
        ResistanceType,
        "New Resistance"
    );
    test_unknown_roundtrip!(test_component_type_unknown, ComponentType, "New Component");
    test_unknown_roundtrip!(test_noise_unknown, Noise, "New Noise");
    test_unknown_roundtrip!(test_projectile_unknown, Projectile, "New Projectile");
    test_unknown_roundtrip!(test_sex_unknown, Sex, "New Sex");
    test_unknown_roundtrip!(test_disposition_unknown, Disposition, "6");
}

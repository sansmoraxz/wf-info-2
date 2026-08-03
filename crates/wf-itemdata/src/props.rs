//! Composition structs for `#[serde(flatten)]` usage.
//!
//! These structs group related optional properties that can be embedded
//! into item types using serde's flatten attribute, reducing boilerplate
//! while maintaining clear property groupings.

use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::common::{Introduced, deserialize_option_number_to_f64};
use crate::components::Component;
use crate::damage::{Attack, DamageBreakdown};
use crate::enums::{Noise, Polarity, ResistanceType, Slot, Trigger};

/// Core identity fields present on every item type.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemIdentityProps {
    pub unique_name: String,
    pub name: String,
    pub category: String,
}

/// Tradability and mastery properties.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradableProps {
    pub tradable: bool,
    #[serde(default)]
    pub masterable: bool,
}

/// Optional detail fields (image and description).
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetailProps {
    pub image_name: Option<String>,
    pub description: Option<String>,
}

/// Properties for buildable/craftable items.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildableProps {
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub skip_build_time_price: Option<i64>,
    pub consume_on_build: Option<bool>,
    pub mastery_req: Option<i64>,
    pub market_cost: Option<i64>,
    pub bp_cost: Option<i64>,
    #[serde(default)]
    pub components: Vec<Component>,
}

/// Properties for wiki/external resource integration.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WikiaProps {
    pub wiki_available: Option<bool>,
    pub wikia_url: Option<String>,
    pub wikia_thumbnail: Option<String>,
    pub introduced: Option<Introduced>,
    pub release_date: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Properties for Prime items and vault status.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimeProps {
    #[serde(default)]
    pub is_prime: bool,
    pub vaulted: Option<bool>,
    pub vault_date: Option<String>,
    pub estimated_vault_date: Option<String>,
}

/// Properties for equippable items (mod slots, polarities).
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquippableProps {
    #[serde(default)]
    pub polarities: Vec<Polarity>,
    pub slot: Option<Slot>,
}

/// Base weapon stats shared by all weapon types (matches `Weapon` trait).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponProps {
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    pub damage: Option<DamageBreakdown>,
    #[serde(default)]
    pub damage_per_shot: Vec<f64>,
    pub disposition: Option<i64>,
    pub fire_rate: f64,
    pub omega_attenuation: f64,
    pub proc_chance: f64,
    pub total_damage: f64,
    #[serde(default)]
    pub attacks: Vec<Attack>,
}

/// Properties specific to ranged weapons.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GunProps {
    pub accuracy: f64,
    pub multishot: i64,
    #[serde(default)]
    pub noise: Noise,
    #[serde(default)]
    pub trigger: Trigger,
    pub magazine_size: Option<i64>,
    pub reload_time: f64,
    pub projectile: Option<String>,
    pub flight: Option<i64>,
}

/// Properties specific to melee weapons.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeleeProps {
    pub blocking_angle: Option<i64>,
    pub combo_duration: Option<i64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub follow_through: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub range: Option<f64>,

    #[serde(default)]
    pub stance_polarity: Option<Polarity>,

    pub slam_attack: Option<i64>,
    pub slam_radial_damage: Option<i64>,
    pub slam_radius: Option<i64>,
    pub slide_attack: Option<i64>,
    pub heavy_attack_damage: Option<i64>,
    pub heavy_slam_attack: Option<i64>,
    pub heavy_slam_radial_damage: Option<i64>,
    pub heavy_slam_radius: Option<i64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub wind_up: Option<f64>,
}

/// Weapon stats for component items that are weapons (e.g., Prime weapon parts).
///
/// `total_damage` is required (non-optional) and serves as the discriminant
/// for [`ComponentWeapon`]'s untagged enum deserialization.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentWeaponData {
    pub total_damage: f64,

    #[serde(default)]
    pub damage_per_shot: Vec<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub critical_chance: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub critical_multiplier: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub proc_chance: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub fire_rate: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub omega_attenuation: Option<f64>,

    pub damage: Option<DamageBreakdown>,

    #[serde(default)]
    pub attacks: Vec<Attack>,

    pub disposition: Option<i64>,

    // Gun-specific
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub accuracy: Option<f64>,

    #[serde(default)]
    pub noise: Option<Noise>,

    #[serde(default)]
    pub trigger: Option<Trigger>,

    pub magazine_size: Option<i64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub reload_time: Option<f64>,

    pub multishot: Option<i64>,
}

/// Weapon classification for components.
///
/// Components can be weapon parts (with full weapon stats) or simple materials.
/// Uses serde untagged: tries `Armed` first (requires `totalDamage`), falls back to `Unarmed`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComponentWeapon {
    Armed(Box<ComponentWeaponData>),
    // Empty struct variant so untagged matches any map; a unit variant would only match null.
    Unarmed {},
}

impl Default for ComponentWeapon {
    fn default() -> Self {
        Self::Unarmed {}
    }
}

impl ComponentWeapon {
    #[must_use]
    pub fn as_armed(&self) -> Option<&ComponentWeaponData> {
        match self {
            Self::Armed(data) => Some(data),
            Self::Unarmed {} => None,
        }
    }
}

/// Computed weapon type classification.
///
/// This enum represents whether an item has ranged (gun) or melee weapon stats.
/// It's computed from the presence of type-specific fields, not directly deserialized.
#[derive(Debug, Clone, PartialEq, Default, derive_more::IsVariant)]
pub enum WeaponTypeStats {
    /// Ranged weapon (gun) - has magazine, reload, trigger, noise
    Ranged(RangedWeaponData),
    /// Melee weapon - has blocking angle, combo, stance, slam attacks
    Melee(MeleeWeaponData),
    /// No weapon-type-specific stats (simple component or non-weapon)
    #[default]
    None,
}

/// Data specific to ranged weapons.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RangedWeaponData {
    pub accuracy: Option<f64>,
    pub magazine_size: Option<i64>,
    pub reload_time: Option<f64>,
    pub multishot: Option<i64>,
    pub noise: Option<Noise>,
    pub trigger: Option<Trigger>,
    pub projectile: Option<String>,
    pub flight: Option<i64>,
}

/// Data specific to melee weapons.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeleeWeaponData {
    pub blocking_angle: Option<i64>,
    pub combo_duration: Option<i64>,
    pub follow_through: Option<f64>,
    pub range: Option<f64>,
    pub stance_polarity: Option<Polarity>,
    pub slam_attack: Option<i64>,
    pub slam_radial_damage: Option<i64>,
    pub slam_radius: Option<i64>,
    pub slide_attack: Option<i64>,
    pub heavy_attack_damage: Option<i64>,
    pub heavy_slam_attack: Option<i64>,
    pub heavy_slam_radial_damage: Option<i64>,
    pub heavy_slam_radius: Option<i64>,
    pub wind_up: Option<f64>,
}

impl WeaponTypeStats {
    /// Determine weapon type from the presence of type-specific fields.
    ///
    /// Detection logic:
    /// - Has melee-specific fields (blocking_angle, stance_polarity, combo_duration) → Melee
    /// - Has ranged-specific fields (magazine_size, reload_time) → Ranged
    /// - Neither → None
    #[allow(
        clippy::too_many_arguments,
        reason = "mirrors the full set of type-specific weapon fields extracted from deserialized data"
    )]
    #[must_use]
    pub fn detect(
        // Ranged-specific
        accuracy: Option<f64>,
        magazine_size: Option<i64>,
        reload_time: Option<f64>,
        multishot: Option<i64>,
        noise: Option<Noise>,
        trigger: Option<Trigger>,
        projectile: Option<String>,
        flight: Option<i64>,
        // Melee-specific
        blocking_angle: Option<i64>,
        combo_duration: Option<i64>,
        follow_through: Option<f64>,
        range: Option<f64>,
        stance_polarity: Option<Polarity>,
        slam_attack: Option<i64>,
        slam_radial_damage: Option<i64>,
        slam_radius: Option<i64>,
        slide_attack: Option<i64>,
        heavy_attack_damage: Option<i64>,
        heavy_slam_attack: Option<i64>,
        heavy_slam_radial_damage: Option<i64>,
        heavy_slam_radius: Option<i64>,
        wind_up: Option<f64>,
    ) -> Self {
        // Check for melee-specific fields first (more distinctive)
        let has_melee = blocking_angle.is_some()
            || stance_polarity.is_some()
            || combo_duration.is_some()
            || slam_attack.is_some()
            || heavy_attack_damage.is_some();

        // Check for ranged-specific fields
        let has_ranged = magazine_size.is_some() || reload_time.is_some();

        if has_melee {
            Self::Melee(MeleeWeaponData {
                blocking_angle,
                combo_duration,
                follow_through,
                range,
                stance_polarity,
                slam_attack,
                slam_radial_damage,
                slam_radius,
                slide_attack,
                heavy_attack_damage,
                heavy_slam_attack,
                heavy_slam_radial_damage,
                heavy_slam_radius,
                wind_up,
            })
        } else if has_ranged {
            Self::Ranged(RangedWeaponData {
                accuracy,
                magazine_size,
                reload_time,
                multishot,
                noise,
                trigger,
                projectile,
                flight,
            })
        } else {
            Self::None
        }
    }

    /// Get ranged weapon data if available
    #[must_use]
    pub fn as_ranged(&self) -> Option<&RangedWeaponData> {
        match self {
            Self::Ranged(data) => Some(data),
            _ => None,
        }
    }

    /// Get melee weapon data if available
    #[must_use]
    pub const fn as_melee(&self) -> Option<&MeleeWeaponData> {
        match self {
            Self::Melee(data) => Some(data),
            _ => None,
        }
    }
}

/// Base defensive stats.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseStats {
    pub health: i64,
    pub shield: i64,
    pub armor: i64,
}

/// Character/suit stats (Warframe, Archwing, companions).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterStats {
    #[serde(flatten)]
    pub defense: DefenseStats,
    pub power: i64,
    pub stamina: i64,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub sprint: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub sprint_speed: Option<f64>,
}

impl Deref for CharacterStats {
    type Target = DefenseStats;
    fn deref(&self) -> &DefenseStats {
        &self.defense
    }
}

/// Damage type affector within a resistance entry.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Affector {
    pub element: String,
    pub modifier: f64,
}

/// Resistance entry (e.g., Shield, Alloy Armor, Robotic).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resistance {
    pub amount: f64,
    #[serde(rename = "type")]
    pub type_field: ResistanceType,
    #[serde(default)]
    pub affectors: Vec<Affector>,
}

/// Combat stats for enemies.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnemyCombatStats {
    #[serde(flatten)]
    pub defense: DefenseStats,
    pub region_bits: i64,
    #[serde(default)]
    pub resistances: Vec<Resistance>,
}

impl Deref for EnemyCombatStats {
    type Target = DefenseStats;
    fn deref(&self) -> &DefenseStats {
        &self.defense
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buildable_props() {
        let json = r#"{
            "buildPrice": 15000,
            "buildTime": 86400,
            "skipBuildTimePrice": 35,
            "components": []
        }"#;

        let props: BuildableProps = serde_json::from_str(json).unwrap();
        assert_eq!(props.build_price, Some(15000));
        assert_eq!(props.build_time, Some(86400));
    }

    #[test]
    fn test_prime_props() {
        let json = r#"{
            "isPrime": true,
            "vaulted": true,
            "vaultDate": "2021-05-25"
        }"#;

        let props: PrimeProps = serde_json::from_str(json).unwrap();
        assert!(props.is_prime);
        assert!(props.vaulted.unwrap());
    }

    #[test]
    fn test_weapon_props() {
        let json = r#"{
            "criticalChance": 0.2,
            "criticalMultiplier": 2.0,
            "fireRate": 8.0,
            "omegaAttenuation": 1.0,
            "procChance": 0.14,
            "totalDamage": 100.0
        }"#;

        let props: WeaponProps = serde_json::from_str(json).unwrap();
        assert!((props.critical_chance - 0.2).abs() < f64::EPSILON);
        assert!((props.fire_rate - 8.0).abs() < f64::EPSILON);
        assert!(props.damage.is_none());
        assert!(props.disposition.is_none());
    }

    #[test]
    fn test_melee_props() {
        let json = r#"{
            "blockingAngle": 55,
            "comboDuration": 5,
            "followThrough": 0.6,
            "range": 2.5,
            "stancePolarity": "naramon"
        }"#;

        let props: MeleeProps = serde_json::from_str(json).unwrap();
        assert_eq!(props.blocking_angle, Some(55));
        assert_eq!(props.stance_polarity, Some(Polarity::Naramon));
    }

    #[test]
    fn test_weapon_type_stats_ranged() {
        let stats = WeaponTypeStats::detect(
            Some(28.6_f64), // accuracy
            Some(45),       // magazine_size
            Some(2.0_f64),  // reload_time
            Some(1),        // multishot
            Some(Noise::Alarming),
            Some(Trigger::Auto),
            Some("Hitscan".to_owned()), // projectile
            None,                       // flight
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert!(stats.is_ranged());
        assert!(!stats.is_melee());

        let ranged_data = stats.as_ranged().unwrap();
        assert_eq!(ranged_data.magazine_size, Some(45));
        assert_eq!(ranged_data.reload_time, Some(2.0_f64));
        assert_eq!(ranged_data.noise, Some(Noise::Alarming));
    }

    #[test]
    fn test_weapon_type_stats_melee() {
        let stats = WeaponTypeStats::detect(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(55),                // blocking_angle
            Some(5),                 // combo_duration
            Some(0.6_f64),           // follow_through
            Some(2.5_f64),           // range
            Some(Polarity::Naramon), // stance_polarity
            Some(150),               // slam_attack
            Some(100),               // slam_radial_damage
            Some(5),                 // slam_radius
            Some(120),               // slide_attack
            Some(300),               // heavy_attack_damage
            Some(450),               // heavy_slam_attack
            Some(300),               // heavy_slam_radial_damage
            Some(8),                 // heavy_slam_radius
            Some(0.8_f64),           // wind_up
        );

        assert!(stats.is_melee());
        assert!(!stats.is_ranged());

        let melee_data = stats.as_melee().unwrap();
        assert_eq!(melee_data.blocking_angle, Some(55));
        assert_eq!(melee_data.stance_polarity, Some(Polarity::Naramon));
        assert_eq!(melee_data.heavy_attack_damage, Some(300));
    }

    #[test]
    fn test_weapon_type_stats_none() {
        let stats = WeaponTypeStats::detect(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
        );

        assert!(!stats.is_ranged());
        assert!(!stats.is_melee());
        assert!(stats.as_ranged().is_none());
        assert!(stats.as_melee().is_none());
    }

    #[test]
    fn test_weapon_type_stats_melee_priority() {
        // When both ranged and melee fields present, melee takes priority
        // (more distinctive fields)
        let stats = WeaponTypeStats::detect(
            Some(28.6_f64), // accuracy (ranged)
            Some(45),       // magazine_size (ranged)
            Some(2.0_f64),  // reload_time (ranged)
            None,
            None,
            None,
            None,
            None,
            Some(55), // blocking_angle (melee)
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        // Melee should win because melee detection happens first
        assert!(stats.is_melee());
        assert!(!stats.is_ranged());
    }
}

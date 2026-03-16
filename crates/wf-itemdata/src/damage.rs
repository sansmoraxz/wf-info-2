//! Unified damage-related types.
//!
//! This module consolidates Damage, Damage2, Damage3, Damage4, Attack, Attack2,
//! Falloff, Falloff2, Slam, and Radial into single unified types.

use serde::{Deserialize, Serialize};

use crate::common::{deserialize_number_to_f64, deserialize_option_number_to_f64};

/// Full damage breakdown with all damage types.
///
/// Uses Option<f64> for all fields to handle:
/// - Missing fields (not all items have all damage types)
/// - Both integer and float values from JSON source
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageBreakdown {
    /// Total damage (sum of all types)
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub total: Option<f64>,

    // Physical damage types
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub impact: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub puncture: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub slash: Option<f64>,

    // Elemental damage types
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub heat: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub cold: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub electricity: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub toxin: Option<f64>,

    // Combined elemental damage types
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub blast: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub radiation: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub gas: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub magnetic: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub viral: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub corrosive: Option<f64>,

    // Special damage types
    #[serde(
        default,
        rename = "void",
        deserialize_with = "deserialize_option_number_to_f64"
    )]
    pub void_damage: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub tau: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub cinematic: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub shield_drain: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub health_drain: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub energy_drain: Option<f64>,
    #[serde(
        default,
        rename = "true",
        deserialize_with = "deserialize_option_number_to_f64"
    )]
    pub true_damage: Option<f64>,
}

/// Weapon attack definition with damage and optional modifiers.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
    pub name: String,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub speed: Option<f64>,

    #[serde(
        default,
        rename = "crit_chance",
        deserialize_with = "deserialize_option_number_to_f64"
    )]
    pub crit_chance: Option<f64>,

    #[serde(
        default,
        rename = "crit_mult",
        deserialize_with = "deserialize_option_number_to_f64"
    )]
    pub crit_mult: Option<f64>,

    #[serde(
        default,
        rename = "status_chance",
        deserialize_with = "deserialize_option_number_to_f64"
    )]
    pub status_chance: Option<f64>,

    #[serde(default, rename = "shot_type")]
    pub shot_type: Option<String>,

    #[serde(default, rename = "shot_speed")]
    pub shot_speed: Option<i64>,

    pub flight: Option<i64>,

    #[serde(default)]
    pub damage: DamageBreakdown,

    pub falloff: Option<Falloff>,

    #[serde(
        default,
        rename = "charge_time",
        deserialize_with = "deserialize_option_number_to_f64"
    )]
    pub charge_time: Option<f64>,

    pub slam: Option<Slam>,

    /// Polymorphic: can be a string label (e.g. "Slide") or a numeric damage value.
    pub slide: Option<serde_json::Value>,

    pub duration: Option<f64>,

    pub radius: Option<f64>,

    pub pellet: Option<Pellet>,
}

/// Damage falloff over distance for ranged weapons.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Falloff {
    #[serde(deserialize_with = "deserialize_number_to_f64")]
    pub start: f64,
    #[serde(deserialize_with = "deserialize_number_to_f64")]
    pub end: f64,
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub reduction: Option<f64>,
}

/// Slam attack information for melee weapons.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slam {
    /// Damage value (can be number as string in source)
    pub damage: String,
    pub radial: Radial,
}

/// Radial damage from slam attacks.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Radial {
    pub damage: String,
    pub radius: Option<i64>,
    pub element: Option<String>,
    pub proc: Option<String>,
}

/// Pellet information for multi-projectile weapons.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pellet {
    pub name: String,
    pub count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_breakdown_with_mixed_types() {
        // Some values are integers, some are floats
        let json = r#"{
            "total": 100,
            "impact": 25.5,
            "puncture": 25,
            "slash": 50.0
        }"#;

        let damage: DamageBreakdown = serde_json::from_str(json).unwrap();
        assert!((damage.total.unwrap() - 100.0).abs() < f64::EPSILON);
        assert!((damage.impact.unwrap() - 25.5).abs() < f64::EPSILON);
        assert!((damage.puncture.unwrap() - 25.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_damage_breakdown_sparse() {
        // Only some damage types present
        let json = r#"{"impact": 50, "heat": 25}"#;

        let damage: DamageBreakdown = serde_json::from_str(json).unwrap();
        assert!(damage.impact.is_some());
        assert!(damage.heat.is_some());
        assert!(damage.slash.is_none());
        assert!(damage.cold.is_none());
    }

    #[test]
    fn test_attack_deserialize() {
        let json = r#"{
            "name": "Normal Attack",
            "speed": 1.0,
            "crit_chance": 0.2,
            "crit_mult": 2.0,
            "status_chance": 0.3,
            "damage": {"impact": 50, "slash": 30}
        }"#;

        let attack: Attack = serde_json::from_str(json).unwrap();
        assert_eq!(attack.name, "Normal Attack");
        assert!((attack.crit_chance.unwrap() - 0.2).abs() < f64::EPSILON);
    }

    #[test]
    fn test_falloff_with_int_start() {
        let json = r#"{"start": 10, "end": 20.5, "reduction": 0.5}"#;

        let falloff: Falloff = serde_json::from_str(json).unwrap();
        assert!((falloff.start - 10.0).abs() < f64::EPSILON);
        assert!((falloff.end - 20.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_slam_deserialize() {
        let json = r#"{
            "damage": "500",
            "radial": {"damage": "250", "radius": 5}
        }"#;

        let slam: Slam = serde_json::from_str(json).unwrap();
        assert_eq!(slam.damage, "500");
        assert_eq!(slam.radial.radius, Some(5));
    }
}

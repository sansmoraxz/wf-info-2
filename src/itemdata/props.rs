//! Composition structs for `#[serde(flatten)]` usage.
//!
//! These structs group related optional properties that can be embedded
//! into item types using serde's flatten attribute, reducing boilerplate
//! while maintaining clear property groupings.

use serde::{Deserialize, Serialize};

use crate::itemdata::common::{deserialize_option_number_to_f64, Introduced};
use crate::itemdata::components::Component;
use crate::itemdata::damage::DamageBreakdown;
use crate::itemdata::enums::{Noise, Polarity, Trigger};

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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimeProps {
    #[serde(default)]
    pub is_prime: bool,
    pub vaulted: Option<bool>,
    pub vault_date: Option<String>,
    pub estimated_vault_date: Option<String>,
}

/// Properties for equippable items (mod slots, polarities).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquippableProps {
    #[serde(default)]
    pub polarities: Vec<Polarity>,
    pub slot: Option<i64>,
}

/// Base weapon attack properties.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponProps {
    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub accuracy: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub critical_chance: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub critical_multiplier: Option<f64>,

    pub damage: Option<DamageBreakdown>,

    #[serde(default)]
    pub damage_per_shot: Vec<f64>,

    pub disposition: Option<i64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub fire_rate: Option<f64>,

    pub multishot: Option<i64>,

    #[serde(default)]
    pub noise: Option<Noise>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub omega_attenuation: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub proc_chance: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub total_damage: Option<f64>,

    #[serde(default)]
    pub trigger: Option<Trigger>,
}

/// Properties specific to ranged weapons (guns).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GunProps {
    pub magazine_size: Option<i64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub reload_time: Option<f64>,

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

/// Character/suit stats (Warframe, Archwing, companions).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterStats {
    pub health: i64,
    pub shield: i64,
    pub armor: i64,
    pub power: i64,
    pub stamina: i64,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub sprint: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub sprint_speed: Option<f64>,
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
            "accuracy": 28.6,
            "criticalChance": 0.2,
            "criticalMultiplier": 2.0,
            "fireRate": 8.0,
            "procChance": 0.14
        }"#;

        let props: WeaponProps = serde_json::from_str(json).unwrap();
        assert!((props.accuracy.unwrap() - 28.6).abs() < f64::EPSILON);
        assert!((props.critical_chance.unwrap() - 0.2).abs() < f64::EPSILON);
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
}

//! Unified Component type for item crafting requirements.
//!
//! Components can represent simple materials or full weapons (for Prime items).
//! This unified type includes all possible fields with appropriate optionality.

use serde::{Deserialize, Serialize};

use crate::itemdata::common::{Drop, Introduced, deserialize_option_number_to_f64};
use crate::itemdata::damage::{Attack, DamageBreakdown};
use crate::itemdata::enums::{Noise, Polarity, Slot, Trigger};
use crate::itemdata::props::WeaponTypeStats;

/// Crafting component for buildable items.
///
/// This type handles both simple components (like Neurodes) and complex
/// components (like Prime weapon parts that have full weapon stats).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    // Core identity
    pub unique_name: String,
    pub name: String,
    pub item_count: i64,
    pub image_name: String,
    pub tradable: bool,
    pub masterable: bool,

    #[serde(default)]
    pub drops: Vec<Drop>,

    pub description: Option<String>,

    #[serde(rename = "type")]
    pub type_field: Option<String>,

    // Prime trading
    pub prime_selling_price: Option<i64>,
    pub ducats: Option<i64>,

    // Weapon stats (for weapon components)
    #[serde(default)]
    pub damage_per_shot: Vec<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub total_damage: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub critical_chance: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub critical_multiplier: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub proc_chance: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub fire_rate: Option<f64>,

    pub mastery_req: Option<i64>,
    pub product_category: Option<String>,
    pub slot: Option<Slot>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub accuracy: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub omega_attenuation: Option<f64>,

    #[serde(default)]
    pub noise: Option<Noise>,

    #[serde(default)]
    pub trigger: Option<Trigger>,

    pub magazine_size: Option<i64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub reload_time: Option<f64>,

    pub multishot: Option<i64>,

    pub damage: Option<DamageBreakdown>,

    pub wiki_available: Option<bool>,

    #[serde(default)]
    pub attacks: Vec<Attack>,

    pub market_cost: Option<i64>,
    pub bp_cost: Option<i64>,

    #[serde(default)]
    pub polarities: Vec<Polarity>,

    #[serde(default)]
    pub tags: Vec<String>,

    pub wikia_thumbnail: Option<String>,
    pub wikia_url: Option<String>,

    pub disposition: Option<i64>,

    pub introduced: Option<Introduced>,
    pub release_date: Option<String>,

    // Melee-specific
    pub blocking_angle: Option<i64>,
    pub combo_duration: Option<i64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub follow_through: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_number_to_f64")]
    pub range: Option<f64>,

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

    #[serde(default)]
    pub stance_polarity: Option<Polarity>,

    pub exclude_from_codex: Option<bool>,

    pub vaulted: Option<bool>,
    pub estimated_vault_date: Option<String>,
    pub vault_date: Option<String>,
}

impl Component {
    /// Get the computed weapon type classification.
    ///
    /// Returns `WeaponTypeStats::Ranged` for gun components,
    /// `WeaponTypeStats::Melee` for melee weapon components,
    /// or `WeaponTypeStats::None` for simple materials.
    pub fn weapon_type_stats(&self) -> WeaponTypeStats {
        WeaponTypeStats::detect(
            self.accuracy,
            self.magazine_size,
            self.reload_time,
            self.multishot,
            self.noise.clone(),
            self.trigger.clone(),
            None, // projectile - not in Component
            None, // flight - not in Component
            self.blocking_angle,
            self.combo_duration,
            self.follow_through,
            self.range,
            self.stance_polarity.clone(),
            self.slam_attack,
            self.slam_radial_damage,
            self.slam_radius,
            self.slide_attack,
            self.heavy_attack_damage,
            self.heavy_slam_attack,
            self.heavy_slam_radial_damage,
            self.heavy_slam_radius,
            self.wind_up,
        )
    }

    /// Check if this component is a weapon (has weapon-specific stats)
    pub fn is_weapon(&self) -> bool {
        !matches!(self.weapon_type_stats(), WeaponTypeStats::None)
    }

    /// Check if this component is a ranged weapon
    pub fn is_ranged_weapon(&self) -> bool {
        self.weapon_type_stats().is_ranged()
    }

    /// Check if this component is a melee weapon
    pub fn is_melee_weapon(&self) -> bool {
        self.weapon_type_stats().is_melee()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_component() {
        let json = r#"{
            "uniqueName": "/Lotus/Types/Items/MiscItems/Neurodes",
            "name": "Neurodes",
            "itemCount": 2,
            "imageName": "neurodes.png",
            "tradable": false,
            "masterable": false,
            "drops": []
        }"#;

        let component: Component = serde_json::from_str(json).unwrap();
        assert_eq!(component.name, "Neurodes");
        assert_eq!(component.item_count, 2);
        assert!(component.damage.is_none());
    }

    #[test]
    fn test_weapon_component() {
        let json = r#"{
            "uniqueName": "/Lotus/Weapons/Tenno/Rifle/PrimeLasers/PrimeLaserRifle",
            "name": "Lex Prime Barrel",
            "itemCount": 1,
            "imageName": "lex-prime-barrel.png",
            "tradable": true,
            "masterable": false,
            "drops": [],
            "totalDamage": 150,
            "criticalChance": 0.25,
            "ducats": 15,
            "primeSellingPrice": 10
        }"#;

        let component: Component = serde_json::from_str(json).unwrap();
        assert_eq!(component.name, "Lex Prime Barrel");
        assert!(component.total_damage.is_some());
        assert!((component.critical_chance.unwrap() - 0.25).abs() < f64::EPSILON);
        assert_eq!(component.ducats, Some(15));
    }
}

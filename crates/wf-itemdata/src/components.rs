//! Unified Component type for item crafting requirements.
//!
//! Components can represent simple materials or full weapons (for Prime items).
//! This unified type includes all possible fields with appropriate optionality.

use serde::{Deserialize, Serialize};

use crate::common::Drop;
use crate::enums::ComponentType;
use crate::props::{
    ComponentWeapon, EquippableProps, MeleeProps, PrimeProps, TradableProps, WeaponTypeStats,
    WikiaProps,
};

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

    #[serde(flatten)]
    pub trade: TradableProps,

    #[serde(default)]
    pub drops: Vec<Drop>,

    pub description: Option<String>,

    #[serde(rename = "type")]
    pub type_field: Option<ComponentType>,

    // Prime trading
    pub prime_selling_price: Option<i64>,
    pub ducats: Option<i64>,

    pub mastery_req: Option<i64>,
    pub product_category: Option<String>,

    pub market_cost: Option<i64>,
    pub bp_cost: Option<i64>,

    pub exclude_from_codex: Option<bool>,

    #[serde(flatten)]
    pub weapon: ComponentWeapon,

    // Grouped props
    #[serde(flatten)]
    pub wikia: WikiaProps,
    #[serde(flatten)]
    pub prime: PrimeProps,
    #[serde(flatten)]
    pub melee: MeleeProps,
    #[serde(flatten)]
    pub equippable: EquippableProps,
}

impl Component {
    /// Get the computed weapon type classification.
    ///
    /// Returns `WeaponTypeStats::Ranged` for gun components,
    /// `WeaponTypeStats::Melee` for melee weapon components,
    /// or `WeaponTypeStats::None` for simple materials.
    #[must_use]
    pub fn weapon_type_stats(&self) -> WeaponTypeStats {
        let w = self.weapon.as_armed();
        WeaponTypeStats::detect(
            w.and_then(|d| d.accuracy),
            w.and_then(|d| d.magazine_size),
            w.and_then(|d| d.reload_time),
            w.and_then(|d| d.multishot),
            w.and_then(|d| d.noise.clone()),
            w.and_then(|d| d.trigger.clone()),
            None, // projectile - not in Component
            None, // flight - not in Component
            self.melee.blocking_angle,
            self.melee.combo_duration,
            self.melee.follow_through,
            self.melee.range,
            self.melee.stance_polarity.clone(),
            self.melee.slam_attack,
            self.melee.slam_radial_damage,
            self.melee.slam_radius,
            self.melee.slide_attack,
            self.melee.heavy_attack_damage,
            self.melee.heavy_slam_attack,
            self.melee.heavy_slam_radial_damage,
            self.melee.heavy_slam_radius,
            self.melee.wind_up,
        )
    }

    /// Check if this component is a weapon (has weapon-specific stats)
    #[must_use]
    pub fn is_weapon(&self) -> bool {
        !matches!(self.weapon_type_stats(), WeaponTypeStats::None)
    }

    /// Check if this component is a ranged weapon
    #[must_use]
    pub fn is_ranged_weapon(&self) -> bool {
        self.weapon_type_stats().is_ranged()
    }

    /// Check if this component is a melee weapon
    #[must_use]
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
        assert!(component.weapon.as_armed().is_none());
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
        let weapon = component.weapon.as_armed().unwrap();
        assert!((weapon.total_damage - 150.0).abs() < f64::EPSILON);
        assert!((weapon.critical_chance.unwrap() - 0.25).abs() < f64::EPSILON);
        assert_eq!(component.ducats, Some(15));
    }
}

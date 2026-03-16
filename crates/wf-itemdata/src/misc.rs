//! Miscellaneous item data (catch-all category).

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ProductCategory;
use crate::common::{Drop, Patchlog};
use crate::enums::{MiscType, Polarity, Rarity};
use crate::props::{
    BuildableProps, ComponentWeapon, ItemDetailProps, ItemIdentityProps, MeleeProps, PrimeProps,
    TradableProps, WeaponTypeStats, WikiaProps,
};
use crate::traits::{Buildable, Droppable, Item, WikiaLinked};

pub type Root = Vec<Misc>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Misc {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: MiscType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    // Misc-specific
    pub show_in_inventory: Option<bool>,
    pub required: Option<i64>,
    pub standing: Option<i64>,
    pub item_count: Option<i64>,
    pub probability: Option<f64>,
    pub rarity: Option<Rarity>,
    pub reward_name: Option<String>,
    pub tier: Option<i64>,
    pub fusion_points: Option<i64>,
    #[serde(default)]
    pub parents: Vec<String>,
    pub exclude_from_codex: Option<bool>,

    #[serde(flatten)]
    pub weapon: ComponentWeapon,
    #[serde(flatten)]
    pub melee: MeleeProps,

    pub prime_omega_attenuation: Option<f64>,

    // Equippable
    pub slot: Option<i64>,
    #[serde(default)]
    pub polarities: Vec<Polarity>,

    // Prime/vault
    #[serde(flatten)]
    pub prime: PrimeProps,

    pub product_category: Option<String>,

    // Grouped props
    #[serde(flatten)]
    pub build: BuildableProps,
    #[serde(flatten)]
    pub wikia: WikiaProps,

    // Railjack-specific
    pub bin_capacity: Option<i64>,
    pub bin_count: Option<i64>,
    pub capacity_multiplier: Option<Vec<i64>>,
    pub durability: Option<i64>,
    pub fill_rate: Option<f64>,
    pub repair_rate: Option<i64>,
    #[serde(default)]
    pub specialities: Vec<Value>, // NOTE: observed to be empty array or null

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Misc {
    fn get_product_categories(&self) -> Vec<String> {
        match &self.product_category {
            Some(v) => vec![v.clone()],
            None => vec![
                "MiscItems".into(),
                "FusionTreasures".into(),
                "Ships".into(),
                "Drones".into(),
                "CrewShips".into(),
            ],
        }
    }
}

impl Misc {
    /// Get the computed weapon type classification.
    ///
    /// Returns `WeaponTypeStats::Ranged` for gun-like misc items,
    /// `WeaponTypeStats::Melee` for melee-like misc items,
    /// or `WeaponTypeStats::None` for non-weapon items.
    pub fn weapon_type_stats(&self) -> WeaponTypeStats {
        let w = self.weapon.as_armed();
        WeaponTypeStats::detect(
            w.and_then(|d| d.accuracy),
            w.and_then(|d| d.magazine_size),
            w.and_then(|d| d.reload_time),
            w.and_then(|d| d.multishot),
            w.and_then(|d| d.noise.clone()),
            w.and_then(|d| d.trigger.clone()),
            None, // projectile
            None, // flight
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

    /// Check if this misc item is a weapon (has weapon-specific stats)
    pub fn is_weapon(&self) -> bool {
        !matches!(self.weapon_type_stats(), WeaponTypeStats::None)
    }

    /// Check if this misc item is a ranged weapon
    pub fn is_ranged_weapon(&self) -> bool {
        self.weapon_type_stats().is_ranged()
    }

    /// Check if this misc item is a melee weapon
    pub fn is_melee_weapon(&self) -> bool {
        self.weapon_type_stats().is_melee()
    }
}

impl Item for Misc {
    fn unique_name(&self) -> &str {
        &self.identity.unique_name
    }
    fn name(&self) -> &str {
        &self.identity.name
    }
    fn category(&self) -> &str {
        &self.identity.category
    }
    fn type_field(&self) -> &str {
        self.type_field.as_str()
    }
    fn image_name(&self) -> Option<&str> {
        self.detail.image_name.as_deref()
    }
    fn tradable(&self) -> bool {
        self.trade.tradable
    }
    fn masterable(&self) -> bool {
        self.trade.masterable
    }
    fn patchlogs(&self) -> &[Patchlog] {
        &self.patchlogs
    }
}

impl Droppable for Misc {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl Buildable for Misc {
    fn build_price(&self) -> Option<i64> {
        self.build.build_price
    }
    fn build_quantity(&self) -> Option<i64> {
        self.build.build_quantity
    }
    fn build_time(&self) -> Option<i64> {
        self.build.build_time
    }
    fn skip_build_time_price(&self) -> Option<i64> {
        self.build.skip_build_time_price
    }
    fn consume_on_build(&self) -> Option<bool> {
        self.build.consume_on_build
    }
    fn mastery_req(&self) -> Option<i64> {
        self.build.mastery_req
    }
    fn market_cost(&self) -> Option<i64> {
        self.build.market_cost
    }
    fn bp_cost(&self) -> Option<i64> {
        self.build.bp_cost
    }
    fn components(&self) -> &[crate::components::Component] {
        &self.build.components
    }
}

impl WikiaLinked for Misc {
    fn wiki_available(&self) -> Option<bool> {
        self.wikia.wiki_available
    }
    fn wikia_url(&self) -> Option<&str> {
        self.wikia.wikia_url.as_deref()
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        self.wikia.wikia_thumbnail.as_deref()
    }
    fn introduced(&self) -> Option<&crate::common::Introduced> {
        self.wikia.introduced.as_ref()
    }
    fn release_date(&self) -> Option<&str> {
        self.wikia.release_date.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_misc() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/misc_test.json"
        ));

        let rec: Misc = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Gameplay/NarmerSorties/ArchonCrystalBorealMythic"
        );
        assert_eq!(rec.identity.category, "Misc");
        assert_eq!(rec.type_field, MiscType::Misc);
        assert!(!rec.trade.tradable);
        assert!(!rec.trade.masterable);
    }

    #[test]
    fn test_deserialize_misc_archon_shard() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/misc_test_2.json"
        ));
        let rec: Misc = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Gameplay/NarmerSorties/ArchonCrystalBoreal"
        );
        assert_eq!(rec.identity.name, "<Shard_blue_simple> Azure Archon Shard");
        assert_eq!(rec.type_field, MiscType::Misc);
        assert!(!rec.trade.tradable);
    }

    #[test]
    fn test_deserialize_misc_nightwave() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/misc_test_3.json"
        ));
        let rec: Misc = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Accelerator");
        assert_eq!(rec.type_field, MiscType::NightwaveChallenge);
        assert!(!rec.trade.tradable);
    }
}

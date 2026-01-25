use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Warframe frame module
pub mod suit;

/// Warframe primary weapon module
pub mod long_gun;

/// Warframe secondary weapon module
pub mod pistol;

/// Warframe melee module
pub mod melee;

/// Warframe upgrade (mods) module
pub mod upgrades;

/// Warframe archwing module
pub mod space_suit;

/// Warframe archgun module
pub mod space_gun;

/// Warframe archmelee module
pub mod space_melee;

/// Blueprints
pub mod recipe;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FractionSyndicates {
    SteelMeridianSyndicate,
    ArbitersSyndicate,
    CephalonSudaSyndicate,
    PerrinSyndicate,
    RedVeilSyndicate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectId {
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polarity {
    #[serde(rename = "Value")]
    pub value: Option<String>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateWrapper {
    #[serde(rename = "$date")]
    #[serde(deserialize_with = "crate::utils::deserialize_mongo_date_option")]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    /// Warframes
    #[serde(rename = "Suits")]
    pub suits: Vec<suit::Suit>,

    /// Primary Weapons
    #[serde(rename = "LongGuns")]
    pub long_guns: Vec<long_gun::LongGun>,

    /// Secondary Weapons
    #[serde(rename = "Pistols")]
    pub pistols: Vec<pistol::Pistol>,

    /// Melee
    #[serde(rename = "Melee")]
    pub melee: Vec<melee::Melee>,

    /// Archwing
    #[serde(rename = "SpaceSuits")]
    pub space_suits: Vec<space_suit::SpaceSuit>,

    /// Archgun
    #[serde(rename = "SpaceGuns")]
    pub space_guns: Vec<space_gun::SpaceGun>,

    /// ArchMelee
    #[serde(rename = "SpaceMelee")]
    pub space_melee: Vec<space_melee::SpaceMelee>,

    /// Mods + Arcanes (unupgraded)
    #[serde(rename = "RawUpgrades")]
    pub raw_upgrades: Vec<upgrades::RawUpgrade>,

    /// Mods + Arcanes (upgraded)
    #[serde(rename = "Upgrades")]
    pub upgrades: Vec<upgrades::Upgrade>,

    /// Blueprints
    #[serde(rename = "Recipes")]
    pub recipes: Vec<recipe::Recipe>,

    /// Blueprint build in progress
    #[serde(rename = "PendingRecipes")]
    pub pending_recipes: Vec<recipe::PendingRecipe>,

    /// Player remaining trades for the day
    #[serde(rename = "TradesRemaining")]
    pub trades_remaining: Option<i64>,

    /// Syndicate
    #[serde(rename = "SupportedSyndicate")]
    pub supported_syndicates: Option<FractionSyndicates>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json::from_str;

    pub fn load_test_inventory() -> Inventory {
        let inventory_str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/sample_inventory.json"
        ));
        from_str(inventory_str).unwrap()
    }

    #[test]
    fn test_inventory_deserialize() {
        let inventory: Inventory = load_test_inventory();
        assert!(!inventory.suits.is_empty(), "Suits should not be empty");
        assert!(
            !inventory.long_guns.is_empty(),
            "Long guns should not be empty"
        );
        assert!(!inventory.pistols.is_empty(), "Pistols should not be empty");
        assert!(
            !inventory.raw_upgrades.is_empty(),
            "Upgrades should not be empty"
        );
    }
}

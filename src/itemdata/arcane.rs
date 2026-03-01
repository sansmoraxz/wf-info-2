//! Arcane enhancement item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, LevelStat, Patchlog};
use crate::itemdata::enums::{ArcaneType, Rarity};
use crate::itemdata::props::{BuildableProps, ItemDetailProps, ItemIdentityProps, TradableProps};
use crate::itemdata::traits::{Droppable, Item};

pub type Root = Vec<Arcane>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arcane {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: ArcaneType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    // Arcane-specific
    #[serde(default)]
    pub rarity: Option<Rarity>,
    #[serde(default)]
    pub level_stats: Vec<LevelStat>,
    pub exclude_from_codex: Option<bool>,

    // Grouped props
    #[serde(flatten)]
    pub build: BuildableProps,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,

    #[serde(default)]
    pub drops: Vec<Drop>,
}

impl ProductCategory for Arcane {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Upgrades".to_string(), "RawUpgrades".to_string()]
    }
}

impl Item for Arcane {
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

impl Droppable for Arcane {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_arcane() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/arcane_test.json"
        ));

        let rec: Arcane = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Upgrades/CosmeticEnhancers/Defensive/SpeedOnDamage"
        );
        assert_eq!(rec.identity.name, "Arcane Agility");
        assert_eq!(rec.identity.category, "Arcanes");
        assert!(rec.trade.tradable);
        assert!(!rec.trade.masterable);
        assert_eq!(rec.rarity, Some(Rarity::Uncommon));
        assert!(!rec.drops.is_empty());
    }

    #[test]
    fn test_deserialize_arcane_legendary() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/arcane_test_2.json"
        ));
        let rec: Arcane = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Arcane Energize");
        assert_eq!(rec.rarity, Some(Rarity::Legendary));
        assert!(rec.trade.tradable);
        assert_eq!(rec.level_stats.len(), 6);
    }

    #[test]
    fn test_deserialize_arcane_secondary() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/arcane_test_3.json"
        ));
        let rec: Arcane = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Akimbo Slip Shot");
        assert_eq!(rec.rarity, Some(Rarity::Rare));
        assert!(rec.trade.tradable);
    }
}

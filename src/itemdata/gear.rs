//! Consumable gear item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, Patchlog};
use crate::itemdata::enums::GearType;
use crate::itemdata::props::{BuildableProps, ItemDetailProps, ItemIdentityProps, TradableProps};
use crate::itemdata::traits::{Buildable, Droppable, Item};

pub type Root = Vec<Gear>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gear {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: GearType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    // Grouped props
    #[serde(flatten)]
    pub build: BuildableProps,

    // Gear-specific
    pub item_count: Option<i64>,
    pub parents: Option<Vec<String>>,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Gear {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Consumables".to_string()]
    }
}

impl Item for Gear {
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

impl Droppable for Gear {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl Buildable for Gear {
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
    fn components(&self) -> &[crate::itemdata::components::Component] {
        &self.build.components
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_gear() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/gear_test.json"
        ));

        let rec: Gear = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Restoratives/Consumable/MiningLaserC"
        );
        assert_eq!(rec.identity.name, "Advanced Nosam Cutter");
        assert_eq!(rec.identity.category, "Gear");
        assert!(!rec.trade.tradable);
        assert!(!rec.trade.masterable);
        assert_eq!(rec.build.build_price, Some(3500));
        assert_eq!(rec.build.components.len(), 4);
    }

    #[test]
    fn test_deserialize_gear_no_build() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/gear_test_2.json"
        ));
        let rec: Gear = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Restoratives/Consumable/Scanner"
        );
        assert_eq!(rec.identity.name, "Codex Scanner");
        assert_eq!(rec.identity.category, "Gear");
        assert!(!rec.trade.tradable);
        assert_eq!(rec.build.build_price, None);
    }

    #[test]
    fn test_deserialize_gear_with_components() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/gear_test_3.json"
        ));
        let rec: Gear = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Restoratives/LisetAirSupport"
        );
        assert_eq!(rec.identity.name, "Air Support Charges");
        assert_eq!(rec.build.build_price, Some(4000));
        assert_eq!(rec.build.components.len(), 5);
    }
}

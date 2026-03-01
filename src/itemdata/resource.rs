//! Resource item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, Patchlog};
use crate::itemdata::enums::ResourceType;
use crate::itemdata::props::{BuildableProps, ItemDetailProps, ItemIdentityProps, TradableProps};
use crate::itemdata::traits::{Buildable, Droppable, Item};

pub type Root = Vec<Resource>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: ResourceType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    // Grouped props
    #[serde(flatten)]
    pub build: BuildableProps,

    // Resource-specific
    pub item_count: Option<i64>,
    #[serde(default)]
    pub parents: Vec<String>,
    pub exclude_from_codex: Option<bool>,

    // Droppable
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Resource {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["MiscItems".to_string()]
    }
}

impl Item for Resource {
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

impl Droppable for Resource {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl Buildable for Resource {
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
    fn test_deserialize_resource() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/resource_test.json"
        ));

        let rec: Resource = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Items/Gems/Deimos/DeimosCommonOreAItem"
        );
        assert_eq!(rec.identity.name, "Adramalium");
        assert_eq!(rec.identity.category, "Resources");
        assert_eq!(rec.type_field, ResourceType::Gem);
        assert!(!rec.trade.tradable);
        assert!(!rec.trade.masterable);
    }

    #[test]
    fn test_deserialize_resource_type() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/resource_test_2.json"
        ));
        let rec: Resource = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Gameplay/1999Wf/Resources/HexDogTagQuincy"
        );
        assert_eq!(rec.identity.name, "35mm Film");
        assert_eq!(rec.type_field, ResourceType::Resource);
        assert!(!rec.trade.tradable);
    }

    #[test]
    fn test_deserialize_resource_with_drops() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/resource_test_3.json"
        ));
        let rec: Resource = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Advances Debt-Bond");
        assert_eq!(rec.type_field, ResourceType::Resource);
    }
}

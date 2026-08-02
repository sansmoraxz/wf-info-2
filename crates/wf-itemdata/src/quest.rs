//! Quest item data.

use serde::{Deserialize, Serialize};

use crate::ProductCategory;
use crate::common::{Drop, Patchlog};
use crate::enums::QuestType;
use crate::props::{BuildableProps, ItemDetailProps, ItemIdentityProps, TradableProps};
use crate::traits::{Buildable, Droppable, Item};

pub type Root = Vec<Quest>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: QuestType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,
    pub exclude_from_codex: Option<bool>,

    // Grouped props
    #[serde(flatten)]
    pub build: BuildableProps,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Quest {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["QuestKeys".to_string(), "LevelKeys".to_string()]
    }
}

impl Item for Quest {
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
        self.type_field.as_ref()
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

impl Droppable for Quest {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl Buildable for Quest {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_quest() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/quest_test.json"
        ));

        let rec: Quest = from_str(json_data).unwrap();

        assert_eq!(rec.identity.unique_name, "/Lotus/Types/Keys/DojoKey");
        assert_eq!(rec.identity.name, "Clan Key");
        assert_eq!(rec.identity.category, "Quests");
        assert_eq!(rec.type_field, QuestType::Key);
        assert!(!rec.trade.tradable);
        assert!(!rec.trade.masterable);

        // Buildable
        assert_eq!(rec.build.build_price, Some(1500));
        assert_eq!(rec.build.components.len(), 4);
    }

    #[test]
    fn test_deserialize_quest_no_build() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/quest_test_2.json"
        ));
        let rec: Quest = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Keys/GetClemQuest/GetClemQuestKeyChain"
        );
        assert_eq!(rec.identity.name, "A Man Of Few Words");
        assert_eq!(rec.type_field, QuestType::Key);
        assert_eq!(rec.build.build_price, None);
    }

    #[test]
    fn test_deserialize_quest_zariman() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/quest_test_3.json"
        ));
        let rec: Quest = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Angels Of The Zariman");
        assert_eq!(rec.type_field, QuestType::Key);
        assert!(!rec.trade.tradable);
    }
}

//! Fish item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, Patchlog};
use crate::itemdata::props::{ItemDetailProps, ItemIdentityProps, TradableProps};
use crate::itemdata::traits::{Droppable, Item};

pub type Root = Vec<Fish>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fish {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    pub exclude_from_codex: Option<bool>,
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Fish {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Fish".to_string()]
    }
}

impl Item for Fish {
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
        &self.type_field
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

impl Droppable for Fish {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_fish() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/fish_test.json"
        ));

        let rec: Fish = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Items/Fish/Deimos/InfestedCommonDFishItem"
        );
        assert_eq!(rec.identity.name, "Amniophysi");
        assert_eq!(rec.identity.category, "Fish");
        assert_eq!(rec.type_field, "Fish");
        assert!(rec.trade.tradable);
        assert!(!rec.trade.masterable);
        assert!(!rec.drops.is_empty());
    }
}

//! Void Relic item data.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ProductCategory;
use crate::common::{Drop, Patchlog};
use crate::enums::RelicType;
use crate::props::{ItemDetailProps, ItemIdentityProps, TradableProps};
use crate::traits::{Droppable, Item};

pub type Root = Vec<Relic>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relic {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: RelicType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    // Relic-specific
    #[serde(default)]
    pub locations: Vec<Value>, // observed to be empty array
    #[serde(default)]
    pub rewards: Vec<Value>, // observed to be empty array
    pub exclude_from_codex: Option<bool>,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Relic {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["MiscItems".to_string()]
    }
}

impl Item for Relic {
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

impl Droppable for Relic {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_relic() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/relics_test.json"
        ));

        let rec: Relic = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeASilver"
        );
        assert_eq!(rec.identity.name, "Axi P8 Exceptional");
        assert_eq!(rec.identity.category, "Relics");
        assert_eq!(rec.type_field, RelicType::Relic);
        assert!(rec.trade.tradable);
        assert!(!rec.trade.masterable);
        assert!(!rec.drops.is_empty());
    }

    #[test]
    fn test_deserialize_relic_intact() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/relics_test_2.json"
        ));
        let rec: Relic = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Game/Projections/T4VoidProjectionEBronze"
        );
        assert_eq!(rec.identity.name, "Axi A1 Intact");
        assert_eq!(rec.type_field, RelicType::Relic);
        assert!(rec.trade.tradable);
    }

    #[test]
    fn test_deserialize_relic_flawless() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/relics_test_3.json"
        ));
        let rec: Relic = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/Game/Projections/T4VoidProjectionEGold"
        );
        assert_eq!(rec.identity.name, "Axi A1 Flawless");
        assert_eq!(rec.type_field, RelicType::Relic);
        assert!(rec.trade.tradable);
    }
}

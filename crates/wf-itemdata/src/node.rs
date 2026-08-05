//! Mission node item data.

use serde::{Deserialize, Serialize};

use crate::ProductCategory;
use crate::common::{Drop, Patchlog};
use crate::enums::NodeType;
use crate::props::{ItemDetailProps, ItemIdentityProps, TradableProps};
use crate::traits::{Droppable, Item};

pub type Root = Vec<Node>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: NodeType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    // Node-specific
    pub faction_index: i64,
    pub mastery_req: i64,
    pub max_enemy_level: i64,
    pub min_enemy_level: i64,
    pub mission_index: i64,
    pub node_type: i64,
    pub system_index: i64,
    pub system_name: String,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Node {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Node".to_owned()]
    }
}

impl Item for Node {
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

impl Droppable for Node {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_node() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/node_test.json"
        ));

        let rec: Node = from_str(json_data).unwrap();

        assert_eq!(rec.identity.unique_name, "SolNode203");
        assert_eq!(rec.identity.name, "Abaddon");
        assert_eq!(rec.identity.category, "Node");
        assert_eq!(rec.type_field, NodeType::Node);
        assert!(!rec.trade.tradable);
        assert!(!rec.trade.masterable);

        // Node-specific
        assert_eq!(rec.system_name, "Europa");
        assert_eq!(rec.min_enemy_level, 21);
        assert_eq!(rec.max_enemy_level, 23);
        assert!(rec.faction_index >= 0);
        assert!(rec.mission_index >= 0);
    }

    #[test]
    fn test_deserialize_node_hydron() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/node_test_2.json"
        ));
        let rec: Node = from_str(json_data).unwrap();

        assert_eq!(rec.identity.unique_name, "SolNode195");
        assert_eq!(rec.identity.name, "Hydron");
        assert_eq!(rec.system_name, "Sedna");
        assert_eq!(rec.min_enemy_level, 30);
        assert_eq!(rec.max_enemy_level, 40);
    }

    #[test]
    fn test_deserialize_node_mot() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/node_test_3.json"
        ));
        let rec: Node = from_str(json_data).unwrap();

        assert_eq!(rec.identity.unique_name, "SolNode409");
        assert_eq!(rec.identity.name, "Mot");
        assert_eq!(rec.system_name, "Void");
        assert_eq!(rec.min_enemy_level, 40);
        assert_eq!(rec.max_enemy_level, 45);
    }
}

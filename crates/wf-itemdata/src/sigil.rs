//! Sigil item data.

use serde::{Deserialize, Serialize};

use crate::ProductCategory;
use crate::common::{Drop, Patchlog};
use crate::enums::SigilType;
use crate::props::{ItemDetailProps, ItemIdentityProps, TradableProps};
use crate::traits::{Droppable, Item};

pub type Root = Vec<Sigil>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sigil {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: SigilType,
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

impl ProductCategory for Sigil {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["WeaponSkins".to_owned()]
    }
}

impl Item for Sigil {
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

impl Droppable for Sigil {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_sigil() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/sigil_test.json"
        ));

        let rec: Sigil = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Upgrades/Skins/Sigils/Community10YearAnniversarySigil"
        );
        assert_eq!(rec.identity.category, "Sigils");
        assert_eq!(rec.type_field, SigilType::Sigil);
        assert!(!rec.trade.tradable);
        assert!(!rec.trade.masterable);
    }

    #[test]
    fn test_deserialize_sigil_syndicate() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/sigil_test_2.json"
        ));
        let rec: Sigil = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Upgrades/Skins/Sigils/Syndicate/HexRankThree"
        );
        assert_eq!(rec.identity.name, "2-For-1 Sigil");
        assert_eq!(rec.type_field, SigilType::Sigil);
    }

    #[test]
    fn test_deserialize_sigil_conclave() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/sigil_test_3.json"
        ));
        let rec: Sigil = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Accord Sigil");
        assert_eq!(rec.type_field, SigilType::Sigil);
        assert!(!rec.trade.tradable);
    }
}

//! Glyph item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, Patchlog};
use crate::itemdata::enums::GlyphType;
use crate::itemdata::props::{ItemDetailProps, ItemIdentityProps, TradableProps};
use crate::itemdata::traits::{Droppable, Item};

pub type Root = Vec<Glyph>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Glyph {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: GlyphType,
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

impl ProductCategory for Glyph {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Glyphs".to_string()]
    }
}

impl Item for Glyph {
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

impl Droppable for Glyph {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_glyph() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/glyph_test.json"
        ));

        let rec: Glyph = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/StoreItems/AvatarImages/FanChannel/AvatarImageChromaPrimePartner"
        );
        assert_eq!(rec.identity.category, "Glyphs");
        assert_eq!(rec.type_field, GlyphType::Glyph);
        assert!(!rec.trade.tradable);
        assert!(!rec.trade.masterable);
    }

    #[test]
    fn test_deserialize_glyph_fan_channel() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/glyph_test_2.json"
        ));
        let rec: Glyph = from_str(json_data).unwrap();

        assert_eq!(
            rec.identity.unique_name,
            "/Lotus/Types/StoreItems/AvatarImages/FanChannel/AvatarImage13angTV"
        );
        assert_eq!(rec.identity.name, "13angtv Glyph");
        assert_eq!(rec.type_field, GlyphType::Glyph);
    }

    #[test]
    fn test_deserialize_glyph_event() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/glyph_test_3.json"
        ));
        let rec: Glyph = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "1999 Drippy Glyph");
        assert_eq!(rec.type_field, GlyphType::Glyph);
        assert!(!rec.trade.tradable);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Platform {
    Pc,
    Xbox,
    Playstation,
    Nintendo,
    Ios,
    Android,
    Unknown,
}

/// Lossy lookup from the private-use glyph Warframe appends to player
/// names in chat logs; unrecognized glyphs map to `Unknown`.
impl From<&str> for Platform {
    fn from(glyph: &str) -> Self {
        match glyph {
            "\u{e000}" => Self::Pc,
            "\u{e001}" => Self::Xbox,
            "\u{e002}" => Self::Playstation,
            "\u{e003}" => Self::Nintendo,
            "\u{e004}" => Self::Ios,
            "\u{e005}" => Self::Android,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub username: String,
    pub platform: Platform,
    pub clan: String,
}

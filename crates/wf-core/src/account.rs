use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Platform {
    PC,
    XBOX,
    PLAYSTATION,
    NINTENDO,
    IOS,
    ANDROID,
    UNKNOWN,
}

impl Platform {
    /// Lossy lookup from the private-use glyph Warframe appends to player
    /// names in chat logs; unrecognized glyphs map to `UNKNOWN`.
    pub fn from_glyph(glyph: &str) -> Self {
        match glyph {
            "\u{e000}" => Self::PC,
            "\u{e001}" => Self::XBOX,
            "\u{e002}" => Self::PLAYSTATION,
            "\u{e003}" => Self::NINTENDO,
            "\u{e004}" => Self::IOS,
            "\u{e005}" => Self::ANDROID,
            _ => Self::UNKNOWN,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub username: String,
    pub platform: Platform,
    pub clan: String,
}

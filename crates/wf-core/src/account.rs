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

/// An in-game player name as it appears in chat logs (platform glyph stripped).
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Display,
    derive_more::From,
    derive_more::AsRef,
)]
#[serde(transparent)]
#[from(forward)]
#[as_ref(str)]
pub struct Username(String);

impl PartialEq<&str> for Username {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

/// A clan tag as reported in the login log line, in `Name#id` form.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Display,
    derive_more::From,
    derive_more::AsRef,
)]
#[serde(transparent)]
#[from(forward)]
#[as_ref(str)]
pub struct Clan(String);

impl PartialEq<&str> for Clan {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub username: Username,
    pub platform: Platform,
    pub clan: Clan,
}

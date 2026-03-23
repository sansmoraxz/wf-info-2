use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Platform {
    PC = 0,
    XBOX = 1,
    PLAYSTATION = 2,
    NINTENDO = 3,
    IOS = 4,
    ANDROID = 5,
    UNKNOWN = 1999,
}

impl From<&str> for Platform {
    fn from(value: &str) -> Self {
        use Platform::*;
        match value {
            "\u{e000}" => PC,
            "\u{e001}" => XBOX,
            "\u{e002}" => PLAYSTATION,
            "\u{e003}" => NINTENDO,
            "\u{e004}" => IOS,
            "\u{e005}" => ANDROID,
            _ => UNKNOWN,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub username: String,
    pub platform: Platform,
    pub account_id: String,
    pub clan: String,
}

pub mod pattern;
mod source;

use serde::{Deserialize, Serialize};

use crate::account::{AccountInfo, Platform, Username};

#[cfg(windows)]
pub use source::DbwinLogSource;
#[cfg(unix)]
pub use source::WineDebugLogSource;
pub use source::{LineAssembler, LogSource};

#[derive(Debug)]
pub enum LogEvent {
    Login(AccountInfo),
    Logout,
    /// The game received its normal quit command. The process watcher confirms
    /// the actual exit before a public lifecycle event is emitted.
    QuitRequested,
    DmTabOpened(DirectMessageInfo),
    /// The local client issued an `IRC out: WHO <username>` query,
    /// indicating the user initiated a DM conversation.
    WhoQuery(Username),
    TradeConfirmPopup(TradeInfo),
    TradeSuccess,
    /// Trade failed with wrapped reason
    TradeFail(String),
    /// Relic crack countdown start (user should take screenshot and run ocr)
    /// There's no other known way to extract rewards directly yet
    RelicOpen,
    // Relic crack countdown end
    RelicClose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeInfo {
    pub sent: Vec<TradeItem>,
    pub received: Vec<TradeItem>,
    pub name: Username,
    pub platform: Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeItem {
    pub name: String,
    pub count: u32,
}

#[derive(Debug)]
pub struct DirectMessageInfo {
    pub username: Username,
    pub platform: Platform,
}

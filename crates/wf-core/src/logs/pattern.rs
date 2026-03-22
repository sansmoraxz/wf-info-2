use regex::{CaptureMatches, Captures, Match, Regex};

use crate::logs::{LogEvent, TradeItem};

use std::sync::LazyLock;

pub trait RegMatcher {
    fn pattern(&self) -> &Regex;
}

/// Helper to create structs and regex patterns for matching
macro_rules! lgreg {
    ($name:ident, $glb:ident, $pat:expr) => {
        #[allow(non_upper_case_globals, dead_code)]
        static $glb: LazyLock<Regex> = LazyLock::new(|| Regex::new($pat).unwrap());

        #[allow(dead_code)]
        pub struct $name;

        impl RegMatcher for $name {
            fn pattern(&self) -> &Regex {
                &$glb
            }
        }
    };
}

pub trait LogEntryTransformer: RegMatcher {
    fn transform(&self, c: &Captures) -> Option<LogEvent>;
}

lgreg!(
    TradeConfirmEntry,
    TRADE_CONFIRMATION_DIALOG_REGEX,
    r"(?ums)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOkCancel\(description=Are you sure you want to accept this trade\? You are offering:(.*)and will receive from (.*)(.) the following:(.*), leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel\)$"
);

fn trade_confirm_item_filter(a: &str) -> Option<TradeItem> {
    if a.len() == 0 {
        return None;
    }
    if let Some(csplit) = a.rfind(" x ") {
        if let Some(r) = a.get(csplit + 3..) {
            if let Ok(count) = r.parse::<u32>() {
                let l = a.get(..csplit)?;
                println!("l: {}; r: {}", l, r);
                return Some(TradeItem {
                    name: l.to_string(),
                    count: count,
                });
            }
        }
    }
    return Some(TradeItem {
        name: a.to_string(),
        count: 1,
    });
}

impl LogEntryTransformer for TradeConfirmEntry {
    /// G1: sent, G2: other player name, G3: platform ucode, G4: received
    fn transform(&self, c: &Captures<'_>) -> Option<LogEvent> {
        let sent = c
            .get(1)?
            .as_str()
            .lines()
            .filter_map(trade_confirm_item_filter)
            .collect();
        let name = c.get(2)?.as_str().to_string();
        let received = c
            .get(4)?
            .as_str()
            .lines()
            .filter_map(trade_confirm_item_filter)
            .collect();
        let platform = c.get(3)?.as_str().to_string();

        let info = crate::logs::TradeInfo {
            sent,
            received,
            name,
            platform,
        };
        Some(crate::logs::LogEvent::TradeConfirmPopup(info))
    }
}

lgreg!(
    TradeSuccessEntry,
    TRADE_SUCCESS_REGEX,
    r"(?m)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOk\(description=The trade was successful!, leftItem=/Menu/Confirm_Item_Ok\)$"
);

impl LogEntryTransformer for TradeSuccessEntry {
    /// at this point we are sure that the regex pattern matched
    fn transform(&self, _: &Captures<'_>) -> Option<LogEvent> {
        Some(LogEvent::TradeSuccess)
    }
}

lgreg!(
    TradeFailEntry,
    TRADE_FAIL_REGEX,
    r"(?m)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOk\(description=The trade failed: ([\w+\s\.]+), leftItem=/Menu/Confirm_Item_Ok\)$"
);

impl LogEntryTransformer for TradeFailEntry {
    /// G1: reason
    fn transform(&self, c: &Captures<'_>) -> Option<LogEvent> {
        let reason = c.get(1)?.as_str().to_string();
        Some(LogEvent::TradeFail(reason))
    }
}

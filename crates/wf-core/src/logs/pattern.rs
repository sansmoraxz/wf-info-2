use anyhow::Error;
use regex::{Captures, Regex, RegexSet};

use crate::{
    account::AccountInfo,
    logs::{DirectMessageInfo, LogEvent, TradeItem},
};

use std::{collections::VecDeque, sync::LazyLock};

pub trait RegMatcher {
    fn pattern(&self) -> &Regex;
}

pub trait LogEntryTransformer: RegMatcher + Sync + Send {
    fn transform(&self, c: &Captures) -> Option<LogEvent>;
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

pub struct LogProcessingEngine {
    transformers: Vec<Box<dyn LogEntryTransformer>>,
    reset: RegexSet,
}

/// temp struct used for sorting events
struct LogRecords {
    pos: usize,
    event: LogEvent,
}

impl LogProcessingEngine {
    pub fn new() -> Result<Self, Error> {
        let t0 = Box::new(WhoQueryEntry);
        let t1 = Box::new(TradeConfirmEntry);
        let t2 = Box::new(TradeSuccessEntry);
        let t3 = Box::new(TradeFailEntry);
        let t4 = Box::new(LoginEntry);
        let t5 = Box::new(LogoutEntry);
        let t6 = Box::new(DMTabEntry);
        let v: Vec<Box<dyn LogEntryTransformer>> = vec![t0, t1, t2, t3, t4, t5, t6];
        let rv: Vec<&str> = v.iter().map(|p| p.pattern().as_str()).collect();
        let reset: RegexSet = RegexSet::new(rv)?;
        Ok(Self {
            transformers: v,
            reset: reset,
        })
    }

    pub fn extract_events(&self, s: &str) -> Vec<LogEvent> {
        let set = &self.reset;
        let container = &self.transformers;
        let mut v: VecDeque<_> = set
            .matches(s)
            .into_iter()
            .filter_map(move |cap_idx| {
                // Dereference the match index to get the corresponding
                // compiled pattern.
                let g = container[cap_idx]
                    .pattern()
                    .captures_iter(s)
                    .filter_map(move |c| {
                        let pos = c.get(0)?.start();
                        let event = container[cap_idx].transform(&c)?;
                        Some(LogRecords { pos, event })
                    })
                    .into_iter()
                    .collect::<Vec<_>>();
                return Some(g);
            })
            .flatten()
            .collect();

        v.make_contiguous().sort_by(|a, b| a.pos.cmp(&b.pos));
        v.drain(..).map(move |rec| rec.event).collect()
    }
}

lgreg!(
    TradeConfirmEntry,
    TRADE_CONFIRMATION_DIALOG_REGEX,
    r"(?Rums)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOkCancel\(description=Are you sure you want to accept this trade\? You are offering:(.*?)and will receive from (.*?)(.) the following:(.*?), leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel\)$"
);

fn trade_confirm_item_filter(a: &str) -> Option<TradeItem> {
    if a.len() == 0 {
        return None;
    }
    if let Some(csplit) = a.rfind(" x ") {
        if let Some(r) = a.get(csplit + 3..) {
            if let Ok(count) = r.parse::<u32>() {
                let l = a.get(..csplit)?;
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
        let platform = c.get(3)?.as_str().into();

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
    r"(?Rm)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOk\(description=The trade was successful!, leftItem=/Menu/Confirm_Item_Ok\)$"
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
    r"(?Rmu)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOk\(description=The trade failed: (.+?), leftItem=/Menu/Confirm_Item_Ok\)$"
);

impl LogEntryTransformer for TradeFailEntry {
    /// G1: reason
    fn transform(&self, c: &Captures<'_>) -> Option<LogEvent> {
        let reason = c.get(1)?.as_str().to_string();
        Some(LogEvent::TradeFail(reason))
    }
}

lgreg!(
    LoginEntry,
    LOGIN_DETAILS_REGEX,
    r"(?Rum)^\d+\.\d+ Sys \[Info\]: Player name changed to ([\w\.\-]+)??(.) Clan: ([\w -]+)#(\d+) AccountId: (\w+)$"
);

impl LogEntryTransformer for LoginEntry {
    /// G1: name, G2: platform, G3: clan, G4: clan hash, G5: account_id
    fn transform(&self, c: &Captures) -> Option<LogEvent> {
        let name = c.get(1)?.as_str().to_string();
        let platform = c.get(2)?.as_str().into();
        let clan_name = c.get(3)?.as_str().to_string();
        let clan_id = c.get(3)?.as_str().to_string();
        let account_id = c.get(5)?.as_str().to_string();
        let clan = [clan_name, "#".to_string(), clan_id].concat();
        let account_info = AccountInfo {
            username: name,
            platform: platform,
            account_id: account_id,
            clan: clan,
        };
        Some(LogEvent::Login(account_info))
    }
}

lgreg!(
    LogoutEntry,
    LOGOUT_DETAILS_REGEX,
    r"(?Rm)^\d+\.\d+ Net \[Info\]: IRC out: QUIT :Logged out of game$"
);

impl LogEntryTransformer for LogoutEntry {
    /// implicit
    fn transform(&self, _: &Captures) -> Option<LogEvent> {
        Some(LogEvent::Logout)
    }
}

lgreg!(
    DMTabEntry,
    DM_TAB_REGEX,
    r"(?Rmu)^\d+\.\d+ Script \[Info\]: ChatRedux\.lua: ChatRedux::AddTab: Adding tab with channel name: F([\w\.\-]+)(.) to index \d+$"
);

impl LogEntryTransformer for DMTabEntry {
    /// G1: name, G2: platform
    fn transform(&self, c: &Captures) -> Option<LogEvent> {
        let username = c.get(1)?.as_str().to_string();
        let platform = c.get(2)?.as_str().into();
        let dm_info = DirectMessageInfo { username, platform };
        Some(LogEvent::DmTabOpened(dm_info))
    }
}

lgreg!(
    WhoQueryEntry,
    WHO_REGEX,
    r"(?Rmu)^\d+\.\d+ Net \[Info\]: IRC out: WHO ([\w\.\-]+)\?\?\? n%nu$"
);

impl LogEntryTransformer for WhoQueryEntry {
    /// G1: name
    fn transform(&self, c: &Captures) -> Option<LogEvent> {
        let username = c.get(1)?.as_str().to_string();
        Some(LogEvent::WhoQuery(username))
    }
}

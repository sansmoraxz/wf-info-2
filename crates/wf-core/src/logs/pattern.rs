use anyhow::Error;
use regex::{Captures, Regex, RegexSet};

use crate::{
    account::{AccountInfo, Platform},
    logs::{DirectMessageInfo, LogEvent, TradeItem},
};

use std::collections::{HashMap, VecDeque};

type Transform = fn(&Captures) -> Option<LogEvent>;

struct Transformer {
    pattern: Regex,
    transform: Transform,
}

/// Pattern table: (regex, capture-group transform). Group meanings are noted
/// per entry.
fn transformers() -> Result<Vec<Transformer>, Error> {
    let table: [(&str, Transform); 10] = [
        // G1: name
        (
            r"(?Rmu)^\d+\.\d+ Net \[Info\]: IRC out: WHO ([\w\.\-]+)\?\?\? n%nu$",
            transform_who_query,
        ),
        // G1: sent, G2: other player name, G3: platform glyph, G4: received
        (
            r"(?Rums)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOkCancel\(description=Are you sure you want to accept this trade\? You are offering:(.*?)and will receive from (.*?)(.) the following:(.*?), title=[[:ascii:]]*? leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel\)$",
            transform_trade_confirm,
        ),
        (
            r"(?Rm)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOk\(description=The trade was successful!, title=[[:ascii:]]*? leftItem=/Menu/Confirm_Item_Ok\)$",
            |_| Some(LogEvent::TradeSuccess),
        ),
        // G1: reason
        (
            r"(?Rmu)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOk\(description=The trade failed: (.+?), title=[[:ascii:]]*? leftItem=/Menu/Confirm_Item_Ok\)$",
            |c| Some(LogEvent::TradeFail(c.get(1)?.as_str().to_string())),
        ),
        // G1: name, G2: platform glyph, G3: clan, G4: clan hash.
        // A legacy `AccountId:` suffix may be present, but is deliberately ignored.
        (
            r"(?Rum)^\d+\.\d+ Sys \[Info\]: Player name changed to ([\w\.\-]+)(.) Clan: ([\w -]+)#(\d+)(?: AccountId: \w+)?$",
            transform_login,
        ),
        (r"(?Rm)^\d+\.\d+ Sys \[Info\]: Logout confirmed$", |_| {
            Some(LogEvent::Logout)
        }),
        (
            r"(?Rm)^\d+\.\d+ Sys \[Info\]: Executing command: /EE/Editor/ToolMenus/Commands/CmdQuit$",
            |_| Some(LogEvent::QuitRequested),
        ),
        // G1: name, G2: platform glyph
        (
            r"(?Rmu)^\d+\.\d+ Script \[Info\]: ChatRedux\.lua: ChatRedux::AddTab: Adding tab with channel name: F([\w\.\-]+)(.) to index \d+$",
            transform_dm_tab,
        ),
        (
            r"(?Rm)\d+\.\d+ Script \[Info\]: ProjectionRewardChoice\.lua: Got rewards$",
            |_| Some(LogEvent::RelicOpen),
        ),
        (
            r"(?Rm)\d+\.\d+ Script \[Info\]: ProjectionRewardChoice\.lua: Relic reward screen shut down$",
            |_| Some(LogEvent::RelicClose),
        ),
    ];

    table
        .into_iter()
        .map(|(pat, transform)| {
            Ok(Transformer {
                pattern: Regex::new(pat)?,
                transform,
            })
        })
        .collect()
}

fn transform_who_query(c: &Captures) -> Option<LogEvent> {
    Some(LogEvent::WhoQuery(c.get(1)?.as_str().to_string()))
}

fn transform_trade_confirm(c: &Captures) -> Option<LogEvent> {
    let sent = extract_trade_items(c.get(1)?.as_str());
    let name = c.get(2)?.as_str().to_string();
    let received = extract_trade_items(c.get(4)?.as_str());
    let platform = Platform::from_glyph(c.get(3)?.as_str());

    let info = crate::logs::TradeInfo {
        sent,
        received,
        name,
        platform,
    };
    Some(LogEvent::TradeConfirmPopup(info))
}

fn transform_login(c: &Captures) -> Option<LogEvent> {
    let name = c.get(1)?.as_str().to_string();
    let platform = Platform::from_glyph(c.get(2)?.as_str());
    let clan_name = c.get(3)?.as_str().to_string();
    let clan_id = c.get(4)?.as_str().to_string();
    let clan = [clan_name, "#".to_string(), clan_id].concat();
    let account_info = AccountInfo {
        username: name,
        platform,
        clan,
    };
    Some(LogEvent::Login(account_info))
}

fn transform_dm_tab(c: &Captures) -> Option<LogEvent> {
    let username = c.get(1)?.as_str().to_string();
    let platform = Platform::from_glyph(c.get(2)?.as_str());
    Some(LogEvent::DmTabOpened(DirectMessageInfo {
        username,
        platform,
    }))
}

pub struct LogProcessingEngine {
    transformers: Vec<Transformer>,
    reset: RegexSet,
}

/// temp struct used for sorting events
struct LogRecords {
    pos: usize,
    event: LogEvent,
}

impl LogProcessingEngine {
    pub fn new() -> Result<Self, Error> {
        let transformers = transformers()?;
        let reset = RegexSet::new(transformers.iter().map(|t| t.pattern.as_str()))?;
        Ok(Self {
            transformers,
            reset,
        })
    }

    pub fn extract_events(&self, s: &str) -> Vec<LogEvent> {
        let container = &self.transformers;
        let mut v: VecDeque<_> = self
            .reset
            .matches(s)
            .into_iter()
            .flat_map(move |cap_idx| {
                let t = &container[cap_idx];
                t.pattern.captures_iter(s).filter_map(move |c| {
                    let pos = c.get(0)?.start();
                    let event = (t.transform)(&c)?;
                    Some(LogRecords { pos, event })
                })
            })
            .collect();

        v.make_contiguous().sort_by_key(|a| a.pos);
        v.drain(..).map(move |rec| rec.event).collect()
    }
}

fn trade_confirm_item_filter(a: &str) -> Option<(String, u32)> {
    let a = a.trim();
    if a.is_empty() {
        return None;
    }
    if let Some(csplit) = a.rfind(" x ")
        && let Some(r) = a.get(csplit + 3..)
        && let Ok(count) = r.trim().parse::<u32>()
    {
        let l = a.get(..csplit)?;
        return Some((l.trim().to_string(), count));
    }
    Some((a.to_string(), 1))
}

fn extract_trade_items(s: &str) -> Vec<TradeItem> {
    let d: Vec<_> = s.lines().filter_map(trade_confirm_item_filter).collect();
    let mut m: HashMap<String, u32> = HashMap::new();
    for e in d {
        m.entry(e.0).and_modify(|c| *c += e.1).or_insert(e.1);
    }
    m.drain()
        .map(|(name, count)| TradeItem { name, count })
        .collect()
}

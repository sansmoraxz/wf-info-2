use regex::{Captures, Regex, RegexSet};

use crate::{
    account::{AccountInfo, Platform},
    logs::{DirectMessageInfo, LogEvent, TradeInfo, TradeItem},
};

use std::collections::HashMap;

type Transform = fn(&Captures) -> Option<LogEvent>;

struct Transformer {
    pattern: Regex,
    transform: Transform,
}

impl Transformer {
    fn new(pattern: &str, transform: Transform) -> Result<Self, regex::Error> {
        Ok(Self {
            pattern: Regex::new(pattern)?,
            transform,
        })
    }
}

pub struct LogProcessingEngine {
    transformers: Vec<Transformer>,
    prefilter: RegexSet,
}

impl LogProcessingEngine {
    pub fn new() -> Result<Self, regex::Error> {
        let transformers = vec![
            Transformer::new(
                r"(?Rmu)^\d+\.\d+ Net \[Info\]: IRC out: WHO (?<name>[\w\.\-]+)\?\?\? n%nu$",
                |c| Some(LogEvent::WhoQuery(c.name("name")?.as_str().into())),
            )?,
            Transformer::new(
                r"(?Rums)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOkCancel\(description=Are you sure you want to accept this trade\? You are offering:(?<sent>.*?)and will receive from (?<name>.*?)(?<platform>.) the following:(?<received>.*?), title=[[:ascii:]]*? leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel\)$",
                |c| {
                    Some(LogEvent::TradeConfirmPopup(TradeInfo {
                        sent: extract_trade_items(c.name("sent")?.as_str()),
                        received: extract_trade_items(c.name("received")?.as_str()),
                        name: c.name("name")?.as_str().into(),
                        platform: Platform::from(c.name("platform")?.as_str()),
                    }))
                },
            )?,
            Transformer::new(
                r"(?Rm)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOk\(description=The trade was successful!, title=[[:ascii:]]*? leftItem=/Menu/Confirm_Item_Ok\)$",
                |_| Some(LogEvent::TradeSuccess),
            )?,
            Transformer::new(
                r"(?Rmu)^\d+\.\d+ Script \[Info\]: Dialog\.lua: Dialog::CreateOk\(description=The trade failed: (?<reason>.+?), title=[[:ascii:]]*? leftItem=/Menu/Confirm_Item_Ok\)$",
                |c| Some(LogEvent::TradeFail(c.name("reason")?.as_str().to_owned())),
            )?,
            // A legacy `AccountId:` suffix may be present, but is deliberately ignored.
            Transformer::new(
                r"(?Rum)^\d+\.\d+ Sys \[Info\]: Player name changed to (?<name>[\w\.\-]+)(?<platform>.) Clan: (?<clan>[\w -]+)#(?<clan_id>\d+)(?: AccountId: \w+)?$",
                |c| {
                    let clan_name = c.name("clan")?.as_str();
                    let clan_id = c.name("clan_id")?.as_str();
                    Some(LogEvent::Login(AccountInfo {
                        username: c.name("name")?.as_str().into(),
                        platform: Platform::from(c.name("platform")?.as_str()),
                        clan: format!("{clan_name}#{clan_id}").into(),
                    }))
                },
            )?,
            Transformer::new(r"(?Rm)^\d+\.\d+ Sys \[Info\]: Logout confirmed$", |_| {
                Some(LogEvent::Logout)
            })?,
            Transformer::new(
                r"(?Rm)^\d+\.\d+ Sys \[Info\]: Executing command: /EE/Editor/ToolMenus/Commands/CmdQuit$",
                |_| Some(LogEvent::QuitRequested),
            )?,
            Transformer::new(
                r"(?Rmu)^\d+\.\d+ Script \[Info\]: ChatRedux\.lua: ChatRedux::AddTab: Adding tab with channel name: F(?<name>[\w\.\-]+)(?<platform>.) to index \d+$",
                |c| {
                    Some(LogEvent::DmTabOpened(DirectMessageInfo {
                        username: c.name("name")?.as_str().into(),
                        platform: Platform::from(c.name("platform")?.as_str()),
                    }))
                },
            )?,
            Transformer::new(
                r"(?Rm)\d+\.\d+ Script \[Info\]: ProjectionRewardChoice\.lua: Got rewards$",
                |_| Some(LogEvent::RelicOpen),
            )?,
            Transformer::new(
                r"(?Rm)\d+\.\d+ Script \[Info\]: ProjectionRewardChoice\.lua: Relic reward screen shut down$",
                |_| Some(LogEvent::RelicClose),
            )?,
        ];
        let prefilter = RegexSet::new(transformers.iter().map(|t| t.pattern.as_str()))?;
        Ok(Self {
            transformers,
            prefilter,
        })
    }

    #[must_use]
    pub fn extract_events(&self, s: &str) -> Vec<LogEvent> {
        let mut v: Vec<(usize, LogEvent)> = self
            .prefilter
            .matches(s)
            .into_iter()
            .filter_map(|idx| self.transformers.get(idx))
            .flat_map(move |t| {
                t.pattern.captures_iter(s).filter_map(move |c| {
                    let pos = c.get(0)?.start();
                    let event = (t.transform)(&c)?;
                    Some((pos, event))
                })
            })
            .collect();

        v.sort_unstable_by_key(|&(pos, _)| pos);
        v.into_iter().map(|(_, event)| event).collect()
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
        return Some((l.trim().to_owned(), count));
    }
    Some((a.to_owned(), 1))
}

fn extract_trade_items(s: &str) -> Vec<TradeItem> {
    s.lines()
        .filter_map(trade_confirm_item_filter)
        .fold(HashMap::<String, u32>::new(), |mut m, (name, count)| {
            *m.entry(name).or_default() += count;
            m
        })
        .into_iter()
        .map(|(name, count)| TradeItem { name, count })
        .collect()
}

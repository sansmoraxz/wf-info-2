use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlOp {
    Ping,
    InventoryLoad,
    InventoryFilter,
    InventoryMetaGet,
    InventoryStaleUpdate,
    ScreenshotTrigger,
    InventoryRefresh,
    Subscribe,
    WFMarketPrice,
    WFMarketRefresh,
    WfmSignstatus,
    WfmSignin,
    WfmSignout,
}

impl ControlOp {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ping => "ping",
            Self::InventoryLoad => "inventory.load",
            Self::InventoryFilter => "inventory.filter",
            Self::InventoryMetaGet => "inventory.meta.get",
            Self::InventoryStaleUpdate => "inventory.stale.update",
            Self::ScreenshotTrigger => "screenshot.trigger",
            Self::InventoryRefresh => "inventory.refresh",
            Self::Subscribe => "subscribe",
            Self::WFMarketPrice => "wfm.price",
            Self::WFMarketRefresh => "wfm.refresh",
            Self::WfmSignstatus => "wfm.signstatus",
            Self::WfmSignin => "wfm.signin",
            Self::WfmSignout => "wfm.signout",
        }
    }

    pub fn parse(op: &str) -> Result<Self> {
        match op {
            "ping" => Ok(Self::Ping),
            "inventory.load" => Ok(Self::InventoryLoad),
            "inventory.filter" => Ok(Self::InventoryFilter),
            "inventory.meta.get" => Ok(Self::InventoryMetaGet),
            "inventory.stale.update" => Ok(Self::InventoryStaleUpdate),
            "screenshot.trigger" => Ok(Self::ScreenshotTrigger),
            "inventory.refresh" => Ok(Self::InventoryRefresh),
            "subscribe" => Ok(Self::Subscribe),
            "wfm.price" => Ok(Self::WFMarketPrice),
            "wfm.refresh" => Ok(Self::WFMarketRefresh),
            "wfm.signstatus" => Ok(Self::WfmSignstatus),
            "wfm.signin" => Ok(Self::WfmSignin),
            "wfm.signout" => Ok(Self::WfmSignout),
            _ => Err(anyhow!("Unknown operation '{}'", op)),
        }
    }
}

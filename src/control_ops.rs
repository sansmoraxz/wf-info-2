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
            _ => Err(anyhow!("Unknown operation '{}'", op)),
        }
    }
}


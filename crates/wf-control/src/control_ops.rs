#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
pub enum ControlOp {
    #[strum(serialize = "ping")]
    Ping,
    #[strum(serialize = "inventory.load")]
    InventoryLoad,
    #[strum(serialize = "inventory.filter")]
    InventoryFilter,
    #[strum(serialize = "inventory.meta.get")]
    InventoryMetaGet,
    #[strum(serialize = "inventory.stale.update")]
    InventoryStaleUpdate,
    #[strum(serialize = "screenshot.trigger")]
    ScreenshotTrigger,
    #[strum(serialize = "inventory.refresh")]
    InventoryRefresh,
    #[strum(serialize = "subscribe")]
    Subscribe,
    #[strum(serialize = "wfm.price")]
    WFMarketPrice,
    #[strum(serialize = "wfm.refresh")]
    WFMarketRefresh,
    #[strum(serialize = "wfm.signstatus")]
    WfmSignstatus,
    #[strum(serialize = "wfm.signin")]
    WfmSignin,
    #[strum(serialize = "wfm.signout")]
    WfmSignout,
}

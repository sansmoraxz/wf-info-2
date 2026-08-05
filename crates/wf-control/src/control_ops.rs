use std::str::FromStr;

/// Wire operation names, decomposed by domain along the dotted format
/// (`inventory.load`, `wfm.price`, ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, derive_more::From)]
pub enum ControlOp {
    #[display("ping")]
    Ping,
    #[display("subscribe")]
    Subscribe,
    #[display("inventory.{_0}")]
    Inventory(InventoryOp),
    #[display("screenshot.{_0}")]
    Screenshot(ScreenshotOp),
    #[display("wfm.{_0}")]
    Wfm(WfmOp),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum InventoryOp {
    Load,
    Filter,
    #[strum(serialize = "meta.get")]
    MetaGet,
    #[strum(serialize = "stale.update")]
    StaleUpdate,
    Refresh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ScreenshotOp {
    Trigger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum WfmOp {
    Price,
    Refresh,
    Signstatus,
    Signin,
    Signout,
}

impl FromStr for ControlOp {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ping" => Self::Ping,
            "subscribe" => Self::Subscribe,
            _ => match s.split_once('.') {
                Some(("inventory", leaf)) => Self::Inventory(leaf.parse()?),
                Some(("screenshot", leaf)) => Self::Screenshot(leaf.parse()?),
                Some(("wfm", leaf)) => Self::Wfm(leaf.parse()?),
                _ => return Err(strum::ParseError::VariantNotFound),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_OPS: [(ControlOp, &str); 13] = [
        (ControlOp::Ping, "ping"),
        (ControlOp::Subscribe, "subscribe"),
        (ControlOp::Inventory(InventoryOp::Load), "inventory.load"),
        (
            ControlOp::Inventory(InventoryOp::Filter),
            "inventory.filter",
        ),
        (
            ControlOp::Inventory(InventoryOp::MetaGet),
            "inventory.meta.get",
        ),
        (
            ControlOp::Inventory(InventoryOp::StaleUpdate),
            "inventory.stale.update",
        ),
        (
            ControlOp::Inventory(InventoryOp::Refresh),
            "inventory.refresh",
        ),
        (
            ControlOp::Screenshot(ScreenshotOp::Trigger),
            "screenshot.trigger",
        ),
        (ControlOp::Wfm(WfmOp::Price), "wfm.price"),
        (ControlOp::Wfm(WfmOp::Refresh), "wfm.refresh"),
        (ControlOp::Wfm(WfmOp::Signstatus), "wfm.signstatus"),
        (ControlOp::Wfm(WfmOp::Signin), "wfm.signin"),
        (ControlOp::Wfm(WfmOp::Signout), "wfm.signout"),
    ];

    #[test]
    fn wire_strings_roundtrip() {
        for (op, wire) in ALL_OPS {
            assert_eq!(op.to_string(), wire);
            assert_eq!(wire.parse::<ControlOp>(), Ok(op));
        }
        "nope".parse::<ControlOp>().unwrap_err();
        "inventory.nope".parse::<ControlOp>().unwrap_err();
        "other.load".parse::<ControlOp>().unwrap_err();
    }
}

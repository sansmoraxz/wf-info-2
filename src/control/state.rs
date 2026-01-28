use std::sync::{OnceLock, RwLock};

static CURRENT_ACCOUNT_ID: OnceLock<RwLock<Option<String>>> = OnceLock::new();

fn account_id_store() -> &'static RwLock<Option<String>> {
    CURRENT_ACCOUNT_ID.get_or_init(|| RwLock::new(None))
}

pub fn set_current_account(account_id: Option<String>) {
    if let Ok(mut guard) = account_id_store().write() {
        *guard = account_id;
    }
}

pub(crate) fn current_account() -> Option<String> {
    account_id_store().read().ok().and_then(|g| g.clone())
}

use anyhow::Result;
use std::time::Duration;

use crate::api;
use crate::inventory::Inventory;
use crate::process;

#[derive(Debug)]
pub struct InventoryFetch {
    pub inventory: Inventory,
    pub auth: process::AuthQuery,
}

pub async fn fetch_inventory_from_process(
    account_id: &str,
    pid: u32,
    scan_retries: u32,
    scan_delay: Duration,
) -> Result<Option<InventoryFetch>> {
    let auth =
        match process::scan_memory_for_auth_with_retry(pid, account_id, scan_retries, scan_delay)
            .await?
        {
            Some(auth) => auth,
            None => return Ok(None),
        };

    let (inventory, auth) =
        fetch_inventory_with_nonce_retry(account_id, pid, auth, scan_retries, scan_delay).await?;

    Ok(Some(InventoryFetch { inventory, auth }))
}

async fn fetch_inventory_with_nonce_retry(
    account_id: &str,
    pid: u32,
    auth: process::AuthQuery,
    scan_retries: u32,
    scan_delay: Duration,
) -> Result<(Inventory, process::AuthQuery)> {
    match api::fetch_inventory(&auth).await {
        Ok(inv) => Ok((inv, auth)),
        Err(e) => {
            log::warn!("Fetch inventory failed (will retry with new nonce): {}", e);
            let new_auth =
                process::scan_memory_for_auth_with_retry(pid, account_id, scan_retries, scan_delay)
                    .await?
                    .ok_or_else(|| {
                        anyhow::anyhow!("Retry failed: still could not locate auth data")
                    })?;
            let inv = api::fetch_inventory(&new_auth).await?;
            Ok((inv, new_auth))
        }
    }
}

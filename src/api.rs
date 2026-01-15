use crate::inventory;
use crate::process::AuthQuery;
use crate::profile::ProfileData;

const PLAYER_INFO_URL: &str = "https://api.warframe.com/cdn/getProfileViewingData.php";
const INVENTORY_URL: &str = "https://api.warframe.com/api/inventory.php";

/// Fetches the player's profile data from the Warframe API using the provided account ID.
/// Returns a ProfileData struct on success.
pub async fn fetch_player_profile(account_id: &str) -> Result<ProfileData, reqwest::Error> {
    reqwest::Client::new()
        .get(PLAYER_INFO_URL)
        .query(&[("playerId", account_id)])
        .send()
        .await?
        .json()
        .await
}

/// Fetches the player's full inventory using the authenticated query.
/// Returns the raw JSON response as a serde_json::Value.
pub async fn fetch_inventory(auth: &AuthQuery) -> anyhow::Result<inventory::Inventory> {
    let url = format!("{}{}", INVENTORY_URL, auth.to_query_string());
    log::info!("Fetching inventory from API...");

    let response = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Inventory API returned status: {}",
            response.status()
        ));
    }

    let inventory: inventory::Inventory = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse inventory JSON: {}", e))?;

    log::info!("Successfully fetched inventory data");
    Ok(inventory)
}

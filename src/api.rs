use crate::profile::Root;

const PLAYER_INFO_URL: &str = "https://api.warframe.com/cdn/getProfileViewingData.php?playerId=";

pub async fn fetch_player_profile(account_id: &str) -> Result<Root, reqwest::Error> {
    let url = format!("{}{}", PLAYER_INFO_URL, account_id);
    log::debug!("Fetching profile from: {}", url);
    let response = reqwest::get(&url).await?;
    let root: Root = response.json().await?;
    Ok(root)
}

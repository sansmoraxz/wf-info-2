use std::fs;
use std::path::PathBuf;

const DEFAULT_BASE_URL: &str =
    "https://raw.githubusercontent.com/WFCD/warframe-items/refs/heads/master/data/json/";

/// All JSON files needed for item data indexing.
const FILE_NAMES: &[&str] = &[
    "Warframes.json",
    "Primary.json",
    "Secondary.json",
    "Melee.json",
    "Archwing.json",
    "Arch-Gun.json",
    "Arch-Melee.json",
    "Mods.json",
];

fn base_url() -> String {
    std::env::var("WF_ITEM_DATA_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string())
}

pub fn cache_dir() -> anyhow::Result<PathBuf> {
    let cache_dir =
        dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("Could not find cache directory"))?;
    let dir = cache_dir.join("wf-info-2").join("itemdata");
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

pub fn cached_path(file: &str) -> anyhow::Result<PathBuf> {
    Ok(cache_dir()?.join(file))
}

/// Download and cache all item data JSON files.
///
/// Uses ETag-based conditional requests: stores ETags in `<file>.etag` sidecar
/// files and sends `If-None-Match` on subsequent requests. Files that haven't
/// changed upstream (304) are skipped.
pub async fn update_cache(client: &reqwest::Client) -> anyhow::Result<()> {
    let dir = cache_dir()?;
    let base = base_url();

    for &file in FILE_NAMES {
        let url = format!("{base}{file}");
        let etag_path = dir.join(format!("{file}.etag"));

        let mut req = client.get(&url);

        // Attach saved ETag for conditional request
        if let Ok(etag) = fs::read_to_string(&etag_path) {
            req = req.header("If-None-Match", etag.trim());
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
                    log::info!("itemdata: {file} unchanged");
                    continue;
                }
                if !resp.status().is_success() {
                    log::warn!("itemdata: {} HTTP {}", file, resp.status());
                    continue;
                }

                // Save ETag if present
                if let Some(etag) = resp.headers().get("etag")
                    && let Ok(val) = etag.to_str()
                {
                    let _ = fs::write(&etag_path, val);
                }

                let body = resp.text().await?;
                fs::write(dir.join(file), &body)?;
                log::info!("itemdata: {file} updated");
            }
            Err(e) => {
                log::warn!("itemdata: failed to fetch {file}: {e}");
            }
        }
    }

    Ok(())
}

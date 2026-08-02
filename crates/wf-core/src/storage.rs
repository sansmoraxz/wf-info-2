use aes_gcm::{
    Aes256Gcm,
    Key, // Key is a type alias, but passing &[u8] via from_slice works
    Nonce,
    aead::{Aead, KeyInit},
};
use anyhow::Context;
use chrono::{DateTime, Utc};
use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use crate::profile::ProfileData;
use wf_inventory as inventory;

// Get key from build-time environment variable
// Can be any string now
const RAW_KEY_ENV: &str = env!("WF_PROFILE_KEY");

// AES-128-CBC
const INVENTORY_KEY: [u8; 16] = [
    76, 69, 79, 45, 65, 76, 69, 67, 9, 69, 79, 45, 65, 76, 69, 67,
];
const INVENTORY_IV: [u8; 16] = [
    49, 50, 70, 71, 66, 51, 54, 45, 76, 69, 51, 45, 113, 61, 57, 0,
];

const PROFILE_FILE: &str = "userstats.dat";
const AUTH_TOKEN_FILE: &str = "auth_token.dat";
const INVENTORY_FILE: &str = "inventory_data.dat";

pub fn save_encrypted_profile(profile: &ProfileData) -> anyhow::Result<()> {
    let json = serde_json::to_vec(profile).context("Failed to serialize profile")?;

    // Hash the raw string key to get a 32-byte key
    let mut hasher = Sha256::new();
    hasher.update(RAW_KEY_ENV.as_bytes());
    let key_bytes = hasher.finalize();

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, json.as_ref())
        .map_err(|e| anyhow::anyhow!("Encryption failure: {e}"))?;

    // Store nonce + ciphertext
    let mut final_data = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
    final_data.extend_from_slice(&nonce_bytes);
    final_data.extend_from_slice(&ciphertext);

    let file_path = app_cache_dir()?.join(PROFILE_FILE);
    let mut file = File::create(&file_path).context("Failed to create output file")?;
    file.write_all(&final_data)
        .context("Failed to write to file")?;

    log::info!("Saved encrypted profile to {}", file_path.display());

    Ok(())
}

pub fn delete_profile() -> anyhow::Result<()> {
    let file_path = app_cache_dir()?.join(PROFILE_FILE);

    if file_path.exists() {
        fs::remove_file(&file_path).context("Failed to delete profile file")?;
        log::info!("Deleted profile data at {}", file_path.display());
    }

    Ok(())
}

// AES-256-GCM helpers (shared by profile & auth token)
fn gcm_encrypt(plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut hasher = Sha256::new();
    hasher.update(RAW_KEY_ENV.as_bytes());
    let key_bytes = hasher.finalize();
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| anyhow::anyhow!("Encryption failure: {e}"))?;

    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

fn gcm_decrypt(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    if data.len() < 12 {
        anyhow::bail!("Data too short for AES-256-GCM");
    }
    let mut hasher = Sha256::new();
    hasher.update(RAW_KEY_ENV.as_bytes());
    let key_bytes = hasher.finalize();
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(&data[..12]);
    let plaintext = cipher
        .decrypt(nonce, &data[12..])
        .map_err(|e| anyhow::anyhow!("Decryption failure: {e}"))?;
    Ok(plaintext)
}

/// Auth token storage (AES-256-GCM)
#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokenData {
    pub access_token: String,
    /// Stored as `""` on disk
    /// The v1 API never issues refresh tokens
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub refresh_token: Option<String>,
    pub device_id: String,
    pub client_id: String,
    pub device_name: String,
    pub expires_at: DateTime<Utc>,
}

pub fn save_auth_token(data: &AuthTokenData) -> anyhow::Result<()> {
    let json = serde_json::to_vec(data).context("Failed to serialize auth token")?;
    let encrypted = gcm_encrypt(&json)?;
    let file_path = app_cache_dir()?.join(AUTH_TOKEN_FILE);
    fs::write(&file_path, encrypted).context(format!("Failed to write {file_path:?}"))?;
    log::info!("Saved encrypted auth token to {}", file_path.display());
    Ok(())
}

pub fn read_auth_token() -> anyhow::Result<AuthTokenData> {
    let file_path = app_cache_dir()?.join(AUTH_TOKEN_FILE);
    let data = fs::read(&file_path).context(format!("Failed to read {file_path:?}"))?;
    let plaintext = gcm_decrypt(&data)?;
    serde_json::from_slice(&plaintext).context("Failed to parse auth token JSON")
}

pub fn delete_auth_token() -> anyhow::Result<()> {
    let file_path = app_cache_dir()?.join(AUTH_TOKEN_FILE);
    if file_path.exists() {
        fs::remove_file(&file_path).context(format!("Failed to delete {file_path:?}"))?;
        log::info!("Deleted auth token at {}", file_path.display());
    }
    Ok(())
}

/// Saves inventory data as AES-128-CBC encrypted file.
pub fn save_inventory(inventory: &inventory::Inventory) -> anyhow::Result<()> {
    let app_cache_dir = app_cache_dir()?;

    let ciphertext = encrypt_inventory_bytes(inventory)?;

    let file_path = app_cache_dir.join(INVENTORY_FILE);
    fs::write(&file_path, ciphertext).context(format!("Failed to write {file_path:?}"))?;
    log::info!("Saved encrypted inventory to {}", file_path.display());

    if let Err(e) = touch_inventory_updated(None) {
        log::warn!("Failed to update inventory metadata: {e}");
    }

    Ok(())
}

pub fn app_cache_dir() -> anyhow::Result<PathBuf> {
    let cache_dir =
        dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("Could not find cache directory"))?;
    let app_cache_dir = cache_dir.join("wf-info-2");

    if !app_cache_dir.exists() {
        fs::create_dir_all(&app_cache_dir).context("Failed to create cache directory")?;
    }

    Ok(app_cache_dir)
}

pub fn decrypt_inventory_bytes(data: &[u8]) -> anyhow::Result<inventory::Inventory> {
    use aes::cipher::{BlockDecryptMut, KeyIvInit, block_padding::Pkcs7};
    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

    let mut buf = data.to_vec();
    let cipher = Aes128CbcDec::new(&INVENTORY_KEY.into(), &INVENTORY_IV.into());
    let plaintext = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .map_err(|e| anyhow::anyhow!("Decryption error: {e:?}"))?;

    let inventory =
        serde_json::from_slice(plaintext).context("Failed to parse decrypted inventory JSON")?;
    Ok(inventory)
}

pub fn read_inventory() -> anyhow::Result<inventory::Inventory> {
    let file_path = app_cache_dir()?.join(INVENTORY_FILE);
    let data = fs::read(&file_path).context(format!("Failed to read {file_path:?}"))?;
    decrypt_inventory_bytes(&data)
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct InventoryMeta {
    pub last_updated: Option<DateTime<Utc>>,
    pub last_source: Option<String>,
    pub stale_at: Option<DateTime<Utc>>,
    pub stale_reason: Option<String>,
}

fn inventory_meta_path() -> anyhow::Result<PathBuf> {
    Ok(app_cache_dir()?.join("inventory_meta.json"))
}

pub fn read_inventory_meta() -> anyhow::Result<InventoryMeta> {
    let path = inventory_meta_path()?;
    if !path.exists() {
        return Ok(InventoryMeta::default());
    }
    let raw = fs::read_to_string(&path).context("Failed to read inventory metadata")?;
    let meta = serde_json::from_str(&raw).context("Failed to parse inventory metadata")?;
    Ok(meta)
}

pub fn write_inventory_meta(meta: &InventoryMeta) -> anyhow::Result<()> {
    let path = inventory_meta_path()?;
    let raw =
        serde_json::to_string_pretty(meta).context("Failed to serialize inventory metadata")?;
    fs::write(&path, raw).context("Failed to write inventory metadata")?;
    Ok(())
}

pub fn touch_inventory_updated(source: Option<&str>) -> anyhow::Result<InventoryMeta> {
    let mut meta = read_inventory_meta()?;
    meta.last_updated = Some(Utc::now());
    meta.stale_at = None;
    meta.stale_reason = None;
    if let Some(src) = source {
        meta.last_source = Some(src.to_string());
    }
    write_inventory_meta(&meta)?;
    Ok(meta)
}

pub fn mark_inventory_stale_at(
    stale_at: DateTime<Utc>,
    reason: Option<String>,
) -> anyhow::Result<InventoryMeta> {
    let mut meta = read_inventory_meta()?;
    meta.stale_at = Some(stale_at);
    meta.stale_reason = reason;
    write_inventory_meta(&meta)?;
    Ok(meta)
}

pub fn clear_inventory_stale() -> anyhow::Result<InventoryMeta> {
    let mut meta = read_inventory_meta()?;
    meta.stale_at = None;
    meta.stale_reason = None;
    write_inventory_meta(&meta)?;
    Ok(meta)
}

/// Encrypt inventory bytes using AES-128-CBC with the built-in key/IV.
/// Returns the ciphertext (PKCS7-padded).
pub fn encrypt_inventory_bytes(inventory: &inventory::Inventory) -> anyhow::Result<Vec<u8>> {
    use aes::cipher::{BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
    type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

    let json_bytes = serde_json::to_vec(inventory).context("Failed to serialize inventory")?;
    let block_size = 16;
    let padded_len = ((json_bytes.len() / block_size) + 1) * block_size;
    let mut buffer = vec![0u8; padded_len];
    buffer[..json_bytes.len()].copy_from_slice(&json_bytes);

    let cipher = Aes128CbcEnc::new(&INVENTORY_KEY.into(), &INVENTORY_IV.into());
    let ciphertext = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, json_bytes.len())
        .map_err(|e| anyhow::anyhow!("Encryption error: {e:?}"))?;

    Ok(ciphertext.to_vec())
}

/// Encrypt inventory bytes using a caller-supplied AES-128-CBC key and IV.
pub fn encrypt_inventory_bytes_with_key(
    inventory: &inventory::Inventory,
    key: &[u8; 16],
    iv: &[u8; 16],
) -> anyhow::Result<Vec<u8>> {
    use aes::cipher::{BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
    type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

    let json_bytes = serde_json::to_vec(inventory).context("Failed to serialize inventory")?;
    let block_size = 16;
    let padded_len = ((json_bytes.len() / block_size) + 1) * block_size;
    let mut buffer = vec![0u8; padded_len];
    buffer[..json_bytes.len()].copy_from_slice(&json_bytes);

    let cipher = Aes128CbcEnc::new(key.into(), iv.into());
    let ciphertext = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, json_bytes.len())
        .map_err(|e| anyhow::anyhow!("Encryption error: {e:?}"))?;

    Ok(ciphertext.to_vec())
}

/// Decrypt inventory bytes using a caller-supplied AES-128-CBC key and IV.
pub fn decrypt_inventory_bytes_with_key(
    data: &[u8],
    key: &[u8; 16],
    iv: &[u8; 16],
) -> anyhow::Result<inventory::Inventory> {
    use aes::cipher::{BlockDecryptMut, KeyIvInit, block_padding::Pkcs7};
    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

    let mut buf = data.to_vec();
    let cipher = Aes128CbcDec::new(key.into(), iv.into());
    let plaintext = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .map_err(|e| anyhow::anyhow!("Decryption error: {e:?}"))?;

    let inventory =
        serde_json::from_slice(plaintext).context("Failed to parse decrypted inventory JSON")?;
    Ok(inventory)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, rng};

    fn load_test_inventory() -> inventory::Inventory {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/sample_inventory.json"
        )))
        .unwrap()
    }

    /// Pins the on-disk token format: refresh_token is `""` when absent,
    /// so files written before the Option<String> change stay readable and
    /// files written after stay readable by older builds.
    #[test]
    fn refresh_token_none_round_trips_as_empty_string() {
        let data = AuthTokenData {
            access_token: "jwt".into(),
            refresh_token: None,
            device_id: "dev".into(),
            client_id: "cli".into(),
            device_name: "name".into(),
            expires_at: Utc::now(),
        };
        let value = serde_json::to_value(&data).unwrap();
        assert_eq!(value["refresh_token"], "");

        let parsed: AuthTokenData = serde_json::from_value(value).unwrap();
        assert_eq!(parsed.refresh_token, None);

        let mut value = serde_json::to_value(&data).unwrap();
        value["refresh_token"] = "tok".into();
        let parsed: AuthTokenData = serde_json::from_value(value).unwrap();
        assert_eq!(parsed.refresh_token.as_deref(), Some("tok"));
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip_default_key() {
        let inventory = load_test_inventory();
        let encrypted = encrypt_inventory_bytes(&inventory).expect("encryption should succeed");

        // Ciphertext should not be empty and should differ from plaintext
        assert!(!encrypted.is_empty());
        let json_bytes = serde_json::to_vec(&inventory).unwrap();
        assert_ne!(
            encrypted, json_bytes,
            "ciphertext must differ from plaintext"
        );

        let decrypted = decrypt_inventory_bytes(&encrypted).expect("decryption should succeed");
        assert_eq!(inventory, decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip_random_key() {
        let inventory = load_test_inventory();

        let mut key = [0u8; 16];
        let mut iv = [0u8; 16];
        rng().fill(&mut key);
        rng().fill(&mut iv);

        let encrypted = encrypt_inventory_bytes_with_key(&inventory, &key, &iv)
            .expect("encryption should succeed");

        assert!(!encrypted.is_empty());
        let json_bytes = serde_json::to_vec(&inventory).unwrap();
        assert_ne!(
            encrypted, json_bytes,
            "ciphertext must differ from plaintext"
        );

        let decrypted = decrypt_inventory_bytes_with_key(&encrypted, &key, &iv)
            .expect("decryption should succeed");
        assert_eq!(inventory, decrypted);
    }

    #[test]
    fn test_decrypt_with_wrong_key_fails() {
        let inventory = load_test_inventory();

        let mut key = [0u8; 16];
        let mut iv = [0u8; 16];
        rng().fill(&mut key);
        rng().fill(&mut iv);

        let encrypted = encrypt_inventory_bytes_with_key(&inventory, &key, &iv)
            .expect("encryption should succeed");

        // Use a different random key — decryption should fail
        let mut wrong_key = [0u8; 16];
        rng().fill(&mut wrong_key);
        // Ensure it's actually different
        if wrong_key == key {
            wrong_key[0] ^= 0xFF;
        }

        let result = decrypt_inventory_bytes_with_key(&encrypted, &wrong_key, &iv);
        assert!(result.is_err(), "decryption with wrong key should fail");
    }

    #[test]
    fn test_multiple_random_keys_produce_different_ciphertext() {
        let inventory = load_test_inventory();

        let mut key1 = [0u8; 16];
        let mut iv1 = [0u8; 16];
        rng().fill(&mut key1);
        rng().fill(&mut iv1);

        let mut key2 = [0u8; 16];
        let mut iv2 = [0u8; 16];
        rng().fill(&mut key2);
        rng().fill(&mut iv2);

        let enc1 = encrypt_inventory_bytes_with_key(&inventory, &key1, &iv1).unwrap();
        let enc2 = encrypt_inventory_bytes_with_key(&inventory, &key2, &iv2).unwrap();

        assert_ne!(
            enc1, enc2,
            "different keys should produce different ciphertext"
        );
    }
}

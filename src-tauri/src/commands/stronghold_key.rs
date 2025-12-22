use base64::{engine::general_purpose, Engine as _};
use keyring::Entry;
use rand::RngCore;
use std::fs;
use tauri::{AppHandle, Manager};

const SERVICE: &str = "my-tauri-app";
const ACCOUNT: &str = "stronghold-master-key";

#[tauri::command]
pub fn get_or_create_stronghold_key(app: AppHandle) -> Result<String, String> {
    let entry = Entry::new(SERVICE, ACCOUNT).map_err(|e| e.to_string())?;

    if let Ok(key) = entry.get_password() {
        return Ok(key);
    }

    let mut random = [0u8; 32];
    rand::rng().fill_bytes(&mut random);
    let key = general_purpose::STANDARD.encode(random);

    entry.set_password(&key).map_err(|e| e.to_string())?;

    if let Ok(data_dir) = app.path().app_data_dir() {
        let vault_path = data_dir.join("vault.hold");
        if vault_path.exists() {
            let _ = fs::remove_file(vault_path);
        }
    }

    Ok(key)
}

#[tauri::command]
pub fn reset_stronghold(app: AppHandle) -> Result<(), String> {
    let entry = Entry::new(SERVICE, ACCOUNT).map_err(|e| e.to_string())?;
    let _ = entry.delete_credential();

    if let Ok(data_dir) = app.path().app_data_dir() {
        let vault_path = data_dir.join("vault.hold");
        if vault_path.exists() {
            fs::remove_file(vault_path).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

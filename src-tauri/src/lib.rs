use crate::commands::auth::{login, register};
use crate::config::init;

mod commands;
mod config;
mod db;
mod entity;
mod enums;
mod errors;
mod inputs;
mod responses;
mod services;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init();

    let db = tauri::async_runtime::block_on(db::establish_connection())
        .expect("Failed to connect to database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(db.clone())
        .invoke_handler(tauri::generate_handler![register, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

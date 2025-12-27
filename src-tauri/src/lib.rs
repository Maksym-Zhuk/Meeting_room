use tauri::Manager;

use crate::commands::{
    auth::{login, register},
    organization_members::{
        change_organization_member_role, create_organization_member, delete_organization_member,
        get_all_organization_member, get_organization_member,
    },
    organizations::{
        create_organization, delete_organization, get_organization, get_organization_for_user,
        update_organization,
    },
    // rooms::{create_room, delete_room, get_rooms_for_user, update_room},
    stronghold_key::{get_or_create_stronghold_key, reset_stronghold},
    users::get_user_info,
};
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
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");

            if let Some(parent) = salt_path.parent() {
                std::fs::create_dir_all(parent).ok();
            }

            app.handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(db.clone())
        .invoke_handler(tauri::generate_handler![
            register,
            login,
            get_or_create_stronghold_key,
            reset_stronghold,
            get_user_info,
            // create_room,
            // update_room,
            // delete_room,
            // get_rooms_for_user
            create_organization,
            update_organization,
            delete_organization,
            get_organization_for_user,
            get_organization,
            create_organization_member,
            change_organization_member_role,
            delete_organization_member,
            get_all_organization_member,
            get_organization_member
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

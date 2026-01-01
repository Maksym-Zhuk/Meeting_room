use std::env;

use tauri::Manager;

use crate::config::init;
use crate::{
    commands::{
        auth::{login, register},
        booking_members::{add_booking_member, delete_booking_member, get_all_booking_member},
        bookings::{create_booking, delete_booking, get_all_room_bookings, update_booking},
        organization_members::{
            change_organization_member_role, create_organization_member,
            delete_organization_member, get_all_organization_member, get_organization_member,
        },
        organizations::{
            create_organization, delete_organization, get_organization, get_organization_for_user,
            update_organization,
        },
        rooms::{create_room, delete_room, get_room_for_organization, update_room},
        stronghold_key::{get_or_create_stronghold_key, reset_stronghold},
        users::get_user_info,
    },
    utils::email::EmailService,
};

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

    let email_service = EmailService::new(
        &env::var("SMTP_HOST").expect("SMTP_HOST must be set in .env file"),
        env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set in .env file"),
        env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set in .env file"),
        env::var("FROM_EMAIL").expect("FROM_EMAIL must be set in .env file"),
    )
    .expect("Failed to initialize email service");

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
        .manage(db)
        .manage(email_service)
        .invoke_handler(tauri::generate_handler![
            register,
            login,
            get_or_create_stronghold_key,
            reset_stronghold,
            get_user_info,
            create_room,
            update_room,
            delete_room,
            get_room_for_organization,
            create_organization,
            update_organization,
            delete_organization,
            get_organization_for_user,
            get_organization,
            create_organization_member,
            change_organization_member_role,
            delete_organization_member,
            get_all_organization_member,
            get_organization_member,
            create_booking,
            update_booking,
            delete_booking,
            get_all_room_bookings,
            delete_booking_member,
            get_all_booking_member,
            add_booking_member
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

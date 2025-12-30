use sea_orm::DatabaseConnection;
use tauri::State;

use crate::{
    errors::ErrorResponse,
    inputs::rooms::{CreateRoomInput, UpdateRoomInput},
    responses::{rooms::Room, text::Response},
    services::room,
    utils::jwt::extract_user_id_from_token,
};

#[tauri::command]
pub async fn create_room(
    input: CreateRoomInput,
    token: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Room, ErrorResponse> {
    let user_id = extract_user_id_from_token(&token)?;
    room::create_room(input, user_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn update_room(
    input: UpdateRoomInput,
    token: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Room, ErrorResponse> {
    extract_user_id_from_token(&token)?;
    room::update_room(input, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn delete_room(
    room_id: String,
    token: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Response, ErrorResponse> {
    extract_user_id_from_token(&token)?;
    room::delete_room(room_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn get_room_for_organization(
    organization_id: String,
    token: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Vec<Room>, ErrorResponse> {
    extract_user_id_from_token(&token)?;
    room::get_room_for_organization(organization_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

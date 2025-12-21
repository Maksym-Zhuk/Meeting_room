use sea_orm::DatabaseConnection;
use tauri::State;

use crate::{
    errors::ErrorResponse,
    inputs::{login::LoginInput, register::RegisterInput},
    responses::auth::AuthResponse,
    services::auth,
};

#[tauri::command]
pub async fn register(
    input: RegisterInput,
    db: State<'_, DatabaseConnection>,
) -> Result<AuthResponse, ErrorResponse> {
    auth::register(input, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn login(
    input: LoginInput,
    db: State<'_, DatabaseConnection>,
) -> Result<AuthResponse, ErrorResponse> {
    auth::login(input, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

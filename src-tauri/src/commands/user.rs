use sea_orm::DatabaseConnection;
use tauri::State;

use crate::{
    errors::ErrorResponse, models::user::User, services::user,
    utils::jwt::extract_user_id_from_token,
};

#[tauri::command]
pub async fn get_user_info(
    token: String,
    db: State<'_, DatabaseConnection>,
) -> Result<User, ErrorResponse> {
    let user_id = extract_user_id_from_token(&token)?;
    user::get_user_info(user_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

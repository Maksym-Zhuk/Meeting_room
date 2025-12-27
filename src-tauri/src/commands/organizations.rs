use sea_orm::DatabaseConnection;
use tauri::State;

use crate::{
    errors::ErrorResponse,
    inputs::organizations::{CreateOrganizationInput, UpdateOrganizationInput},
    responses::{organizations::Organization, text::Response},
    services::organizations,
    utils::{jwt::extract_user_id_from_token, validator::validate_input},
};

#[tauri::command]
pub async fn create_organization(
    token: String,
    input: CreateOrganizationInput,
    db: State<'_, DatabaseConnection>,
) -> Result<Organization, ErrorResponse> {
    let user_id = extract_user_id_from_token(&token)?;
    validate_input(&input)?;

    organizations::create_organization(user_id, input, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn update_organization(
    token: String,
    input: UpdateOrganizationInput,
    db: State<'_, DatabaseConnection>,
) -> Result<Response, ErrorResponse> {
    extract_user_id_from_token(&token)?;
    validate_input(&input)?;

    organizations::update_organization(input, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn delete_organization(
    token: String,
    organization_id: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Response, ErrorResponse> {
    extract_user_id_from_token(&token)?;

    organizations::delete_organization(organization_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn get_organization_for_user(
    token: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Vec<Organization>, ErrorResponse> {
    let user_id = extract_user_id_from_token(&token)?;

    organizations::get_organization_for_user(user_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn get_organization(
    token: String,
    organization_id: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Organization, ErrorResponse> {
    extract_user_id_from_token(&token)?;

    organizations::get_organization(organization_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

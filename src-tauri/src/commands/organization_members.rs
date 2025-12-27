use sea_orm::DatabaseConnection;
use tauri::State;

use crate::{
    enums::organization_roles::OrganizationRole,
    errors::ErrorResponse,
    inputs::organization_members::CreateOrganizationMemberInput,
    responses::{organization_members::OrganizationMembers, text::Response, users::User},
    services::organization_members,
    utils::{jwt::extract_user_id_from_token, validator::validate_input},
};

#[tauri::command]
pub async fn create_organization_member(
    token: String,
    input: CreateOrganizationMemberInput,
    db: State<'_, DatabaseConnection>,
) -> Result<OrganizationMembers, ErrorResponse> {
    let user_id = extract_user_id_from_token(&token)?;
    validate_input(&input)?;

    organization_members::create_organization_member(user_id, input, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn change_organization_member_role(
    token: String,
    role: OrganizationRole,
    db: State<'_, DatabaseConnection>,
) -> Result<Response, ErrorResponse> {
    let user_id = extract_user_id_from_token(&token)?;

    organization_members::change_organization_member_role(user_id, role, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn delete_organization_member(
    token: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Response, ErrorResponse> {
    let user_id = extract_user_id_from_token(&token)?;

    organization_members::delete_organization_member(user_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn get_all_organization_member(
    token: String,
    organization_id: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Vec<User>, ErrorResponse> {
    extract_user_id_from_token(&token)?;

    organization_members::get_all_organization_member(organization_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn get_organization_member(
    token: String,
    organization_member_id: String,
    db: State<'_, DatabaseConnection>,
) -> Result<User, ErrorResponse> {
    extract_user_id_from_token(&token)?;

    organization_members::get_organization_member(organization_member_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

use sea_orm::DatabaseConnection;
use tauri::State;

use crate::{
    errors::ErrorResponse,
    inputs::booking_members::CreateBookingMemberInput,
    responses::{booking_members::BookingMembers, text::Response},
    services::booking_members,
    utils::{email::EmailService, jwt::extract_user_id_from_token, validator::validate_input},
};

#[tauri::command]
pub async fn add_booking_member(
    token: String,
    input: CreateBookingMemberInput,
    db: State<'_, DatabaseConnection>,
    email_service: State<'_, EmailService>,
) -> Result<Response, ErrorResponse> {
    extract_user_id_from_token(&token)?;
    validate_input(&input)?;

    booking_members::add_booking_member(input, db.inner(), email_service.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn delete_booking_member(
    token: String,
    booking_member_id: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Response, ErrorResponse> {
    extract_user_id_from_token(&token)?;

    booking_members::delete_booking_member(booking_member_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn get_all_booking_member(
    token: String,
    booking_id: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Vec<BookingMembers>, ErrorResponse> {
    extract_user_id_from_token(&token)?;

    booking_members::get_all_booking_member(booking_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

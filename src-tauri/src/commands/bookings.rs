use sea_orm::DatabaseConnection;
use tauri::State;

use crate::{
    errors::ErrorResponse,
    inputs::bookings::{CreateBookingInput, UpdateBookingInput},
    responses::{bookings::Booking, text::Response},
    services::bookings,
    utils::{jwt::extract_user_id_from_token, validator::validate_input},
};

#[tauri::command]
pub async fn create_booking(
    token: String,
    input: CreateBookingInput,
    db: State<'_, DatabaseConnection>,
) -> Result<Booking, ErrorResponse> {
    let user_id = extract_user_id_from_token(&token)?;
    validate_input(&input)?;

    bookings::create_booking(input, user_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn update_booking(
    token: String,
    input: UpdateBookingInput,
    db: State<'_, DatabaseConnection>,
) -> Result<Response, ErrorResponse> {
    extract_user_id_from_token(&token)?;
    validate_input(&input)?;

    bookings::update_booking(input, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn delete_booking(
    token: String,
    booking_id: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Response, ErrorResponse> {
    extract_user_id_from_token(&token)?;

    bookings::delete_booking(booking_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

#[tauri::command]
pub async fn get_all_room_bookings(
    token: String,
    room_id: String,
    db: State<'_, DatabaseConnection>,
) -> Result<Vec<Booking>, ErrorResponse> {
    extract_user_id_from_token(&token)?;

    bookings::get_all_room_bookings(room_id, db.inner())
        .await
        .map_err(ErrorResponse::from)
}

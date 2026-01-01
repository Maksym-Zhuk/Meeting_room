use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    TransactionTrait,
};
use uuid::Uuid;

use crate::{
    entity::{
        booking_members::ActiveModel as BookingMemberActiveModel,
        bookings::{self, ActiveModel as BookingActiveModel},
    },
    errors::AppError,
    inputs::bookings::{CreateBookingInput, UpdateBookingInput},
    responses::{bookings::Booking, text::Response},
};

pub async fn create_booking(
    input: CreateBookingInput,
    user_id: String,
    db: &DatabaseConnection,
) -> Result<Booking, AppError> {
    let txn = db.begin().await?;

    let new_booking = BookingActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(input.title),
        room_id: Set(Uuid::parse_str(&input.room_id)?),
        start_at: Set(input.start_at),
        end_at: Set(input.end_at),
        created_by: Set(Uuid::parse_str(&user_id)?),
        created_at: Set(Utc::now().timestamp()),
        updated_at: Set(Utc::now().timestamp()),
    };

    let booking = new_booking.insert(&txn).await?;

    let new_booking_member = BookingMemberActiveModel {
        id: Set(Uuid::new_v4()),
        booking_id: Set(booking.id),
        user_id: Set(Uuid::parse_str(&user_id)?),
    };

    new_booking_member.insert(&txn).await?;

    txn.commit().await?;

    Ok(Booking {
        id: booking.id.to_string(),
        room_id: booking.room_id.to_string(),
        title: booking.title,
        created_by: booking.created_by.to_string(),
        start_at: booking.start_at,
        end_at: booking.end_at,
    })
}

pub async fn update_booking(
    input: UpdateBookingInput,
    db: &DatabaseConnection,
) -> Result<Response, AppError> {
    let booking_model = bookings::Entity::find_by_id(Uuid::parse_str(&input.id)?)
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    let mut booking: BookingActiveModel = booking_model.into();

    if let Some(title) = input.title {
        booking.title = Set(title);
    };

    if let Some(start_at) = input.start_at {
        booking.start_at = Set(start_at);
    };

    if let Some(end_at) = input.end_at {
        booking.end_at = Set(end_at);
    };

    booking.update(db).await?;

    Ok(Response {
        message: "Update successfully".to_string(),
    })
}

pub async fn delete_booking(
    booking_id: String,
    db: &DatabaseConnection,
) -> Result<Response, AppError> {
    let booking_model = bookings::Entity::find_by_id(Uuid::parse_str(&booking_id)?)
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    BookingActiveModel::from(booking_model).delete(db).await?;

    Ok(Response {
        message: "Booking deleted successfully".to_string(),
    })
}

pub async fn get_all_room_bookings(
    room_id: String,
    db: &DatabaseConnection,
) -> Result<Vec<Booking>, AppError> {
    let bookings = bookings::Entity::find()
        .filter(bookings::Column::RoomId.eq(Uuid::parse_str(&room_id)?))
        .all(db)
        .await?;

    let result = bookings
        .into_iter()
        .map(|b| Booking {
            id: b.id.to_string(),
            title: b.title,
            room_id: b.room_id.to_string(),
            created_by: b.created_by.to_string(),
            start_at: b.start_at,
            end_at: b.end_at,
        })
        .collect();

    Ok(result)
}

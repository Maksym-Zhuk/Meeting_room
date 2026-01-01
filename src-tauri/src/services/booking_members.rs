use std::env;

use chrono::{TimeZone, Utc};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    TransactionTrait,
};
use uuid::Uuid;

use crate::{
    entity::{
        booking_members::{self, ActiveModel as BookingMemberActiveModel},
        bookings, users,
    },
    errors::AppError,
    inputs::booking_members::CreateBookingMemberInput,
    responses::{booking_members::BookingMembers, text::Response},
    utils::email::EmailService,
};

pub async fn add_booking_member(
    input: CreateBookingMemberInput,
    db: &DatabaseConnection,
    email_service: &EmailService,
) -> Result<Response, AppError> {
    let txn = db.begin().await?;

    let user = users::Entity::find()
        .filter(users::Column::Email.eq(&input.email))
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    let new_booking_member = BookingMemberActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        booking_id: Set(Uuid::parse_str(&input.booking_id)?),
    };

    new_booking_member.insert(&txn).await?;

    let booking = bookings::Entity::find_by_id(Uuid::parse_str(&input.booking_id)?)
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    txn.commit().await?;

    let dt = Utc.timestamp_millis_opt(booking.start_at).unwrap();
    let meeting_date = dt.format("%d.%m.%Y").to_string();
    let meeting_time = dt.format("%H:%M").to_string();

    email_service.send_meeting_invitation(
        &user.email,
        &user.username,
        &booking.title,
        &meeting_date,
        &meeting_time,
    )?;

    Ok(Response {
        message: "Add new member".to_string(),
    })
}

pub async fn delete_booking_member(
    booking_member_id: String,
    db: &DatabaseConnection,
) -> Result<Response, AppError> {
    let booking_member_model =
        booking_members::Entity::find_by_id(Uuid::parse_str(&booking_member_id)?)
            .one(db)
            .await?
            .ok_or_else(|| AppError::Forbidden)?;

    BookingMemberActiveModel::from(booking_member_model)
        .delete(db)
        .await?;

    Ok(Response {
        message: "Booking member deleted successfully".to_string(),
    })
}

pub async fn get_all_booking_member(
    booking_id: String,
    db: &DatabaseConnection,
) -> Result<Vec<BookingMembers>, AppError> {
    let booking_members = booking_members::Entity::find()
        .filter(booking_members::Column::BookingId.eq(Uuid::parse_str(&booking_id)?))
        .all(db)
        .await?;

    let result = booking_members
        .into_iter()
        .map(|bm| BookingMembers {
            id: bm.id.to_string(),
            booking_id: bm.booking_id.to_string(),
            user_id: bm.user_id.to_string(),
        })
        .collect();

    Ok(result)
}

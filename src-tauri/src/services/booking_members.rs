use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use uuid::Uuid;

use crate::{
    entity::{
        booking_members::{self, ActiveModel as BookingMemberActiveModel},
        users,
    },
    errors::AppError,
    inputs::booking_members::CreateBookingMemberInput,
    responses::{booking_members::BookingMembers, text::Response},
};

pub async fn add_booking_member(
    input: CreateBookingMemberInput,
    db: &DatabaseConnection,
) -> Result<Response, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(input.email))
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    let new_booking_member = BookingMemberActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        booking_id: Set(Uuid::parse_str(&input.booking_id)?),
    };

    new_booking_member.insert(db).await?;

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

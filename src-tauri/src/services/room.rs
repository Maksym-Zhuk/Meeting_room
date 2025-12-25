use crate::{
    entity::room::{self, ActiveModel as RoomActiveModel},
    errors::AppError,
    inputs::room::{CreateRoomInput, UpdateRoomInput},
    models::room::Room,
};
use sea_orm::DatabaseConnection;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{ActiveValue::Set, Condition};
use uuid::Uuid;
use validator::Validate;

pub async fn create_room(
    input: CreateRoomInput,
    user_id: String,
    db: &DatabaseConnection,
) -> Result<Room, AppError> {
    input
        .validate()
        .map_err(|e| AppError::validation(e.to_string()))?;

    let new_room = RoomActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(input.title),
        start: Set(input.start),
        end: Set(input.end),
        creator: Set(Uuid::parse_str(&user_id)?),
        admins: Set(input.admins.into()),
        members: Set(input.members.into()),
        ..Default::default()
    };

    let room = new_room.insert(db).await?;

    Ok(Room {
        id: room.id.to_string(),
        title: room.title,
        start: room.start,
        end: room.end,
        creator: room.creator.to_string(),
        members: serde_json::from_value(room.members)?,
        admins: serde_json::from_value(room.admins)?,
    })
}

pub async fn update_room(
    input: UpdateRoomInput,
    user_id: String,
    db: &DatabaseConnection,
) -> Result<Room, AppError> {
    input
        .validate()
        .map_err(|e| AppError::validation(e.to_string()))?;

    let room_uuid = Uuid::parse_str(&input.id)?;
    let user_uuid = Uuid::parse_str(&user_id)?;

    let room_model = room::Entity::find()
        .filter(
            Condition::any()
                .add(room::Column::Creator.eq(user_uuid.to_string()))
                .add(room::Column::Admins.contains(&user_uuid.to_string())),
        )
        .filter(room::Column::Id.eq(room_uuid))
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    let mut room: RoomActiveModel = room_model.into();

    if let Some(title) = input.title {
        room.title = Set(title);
    }
    if let Some(start) = input.start {
        room.start = Set(start);
    }
    if let Some(end) = input.end {
        room.end = Set(end);
    }
    if let Some(admins) = input.admins {
        room.admins = Set(serde_json::to_value(admins)?);
    }
    if let Some(members) = input.members {
        room.members = Set(serde_json::to_value(members)?);
    }

    let updated = room.update(db).await?;

    Ok(Room {
        id: updated.id.to_string(),
        title: updated.title,
        start: updated.start,
        end: updated.end,
        creator: updated.creator.to_string(),
        members: serde_json::from_value(updated.members)?,
        admins: serde_json::from_value(updated.admins)?,
    })
}

pub async fn delete_room(
    room_id: String,
    user_id: String,
    db: &DatabaseConnection,
) -> Result<String, AppError> {
    let room_uuid = Uuid::parse_str(&room_id)?;
    let user_uuid = Uuid::parse_str(&user_id)?;

    let room_model = room::Entity::find()
        .filter(room::Column::Id.eq(room_uuid))
        .filter(
            Condition::any()
                .add(room::Column::Creator.eq(user_uuid.to_string()))
                .add(room::Column::Admins.contains(&user_uuid.to_string())),
        )
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    RoomActiveModel::from(room_model).delete(db).await?;
    Ok("Room successfully deleted".to_string())
}

pub async fn get_rooms_for_user(
    user_id: String,
    db: &DatabaseConnection,
) -> Result<Vec<Room>, AppError> {
    let rooms = room::Entity::find()
        .filter(
            Condition::any()
                .add(room::Column::Members.contains(&user_id))
                .add(room::Column::Admins.contains(&user_id))
                .add(room::Column::Creator.eq(&user_id)),
        )
        .all(db)
        .await?;

    let result = rooms
        .into_iter()
        .map(|r| Room {
            id: r.id.to_string(),
            title: r.title,
            start: r.start,
            end: r.end,
            creator: r.creator.to_string(),
            members: serde_json::from_value(r.members).unwrap_or_default(),
            admins: serde_json::from_value(r.admins).unwrap_or_default(),
        })
        .collect();

    Ok(result)
}

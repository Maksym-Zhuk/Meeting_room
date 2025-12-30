use crate::{
    entity::rooms::{self, ActiveModel as RoomActiveModel},
    errors::AppError,
    inputs::rooms::{CreateRoomInput, UpdateRoomInput},
    responses::{rooms::Room, text::Response},
};
use chrono::Utc;
use sea_orm::DatabaseConnection;
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{ActiveValue::Set, ColumnTrait, QueryFilter};
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
        name: Set(input.name),
        created_by: Set(Uuid::parse_str(&user_id)?),
        organization_id: Set(Uuid::parse_str(&input.organization_id)?),
        created_at: Set(Utc::now().timestamp()),
        updated_at: Set(Utc::now().timestamp()),
    };

    let room = new_room.insert(db).await?;

    Ok(Room {
        id: room.id.to_string(),
        name: room.name,
        organization_id: room.organization_id.to_string(),
        created_by: room.created_by.to_string(),
    })
}

pub async fn update_room(
    input: UpdateRoomInput,
    db: &DatabaseConnection,
) -> Result<Room, AppError> {
    input
        .validate()
        .map_err(|e| AppError::validation(e.to_string()))?;

    let room_model = rooms::Entity::find_by_id(Uuid::parse_str(&input.id)?)
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    let mut room: RoomActiveModel = room_model.into();

    room.name = Set(input.name);

    let updated = room.update(db).await?;

    Ok(Room {
        id: updated.id.to_string(),
        name: updated.name,
        created_by: updated.created_by.to_string(),
        organization_id: updated.organization_id.to_string(),
    })
}

pub async fn delete_room(room_id: String, db: &DatabaseConnection) -> Result<Response, AppError> {
    let room_model = rooms::Entity::find_by_id(Uuid::parse_str(&room_id)?)
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    RoomActiveModel::from(room_model).delete(db).await?;

    Ok(Response {
        message: "Room successfully deleted".to_string(),
    })
}

pub async fn get_room_for_organization(
    organization_id: String,
    db: &DatabaseConnection,
) -> Result<Vec<Room>, AppError> {
    let rooms = rooms::Entity::find()
        .filter(rooms::Column::OrganizationId.eq(Uuid::parse_str(&organization_id)?))
        .all(db)
        .await?;

    let result = rooms
        .into_iter()
        .map(|r| Room {
            id: r.id.to_string(),
            name: r.name,
            created_by: r.created_by.to_string(),
            organization_id: r.organization_id.to_string(),
        })
        .collect();

    Ok(result)
}

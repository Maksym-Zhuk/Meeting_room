use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::{entity::user, errors::AppError, models::user::User};

pub async fn get_user_info(user_id: String, db: &DatabaseConnection) -> Result<User, AppError> {
    let id_uuid = Uuid::parse_str(&user_id)?;
    let user = user::Entity::find_by_id(id_uuid).one(db).await?;

    let user = match user {
        Some(u) => u,
        None => return Err(AppError::InvalidCredentials.into()),
    };

    Ok(User {
        username: user.username,
        email: user.email,
    })
}

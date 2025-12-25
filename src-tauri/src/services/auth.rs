use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    entity::users::{self, ActiveModel as NewUser},
    enums::role::Role,
    errors::AppError,
    inputs::{login::LoginInput, register::RegisterInput},
    responses::auth::AuthResponse,
    responses::users::User,
    utils::{
        hash::{hash_password, verify_password},
        jwt::generate_token,
    },
};

pub async fn register(
    input: RegisterInput,
    db: &DatabaseConnection,
) -> Result<AuthResponse, AppError> {
    input
        .validate()
        .map_err(|e| AppError::validation(e.to_string()))?;

    let password_hash = hash_password(&input.password)?;

    let txn = db.begin().await?;

    let new_user = NewUser {
        id: Set(Uuid::new_v4()),
        email: Set(input.email),
        username: Set(input.username),
        password: Set(password_hash),
        role: Set("user".to_string()),
        ..Default::default()
    };

    let user = new_user.insert(&txn).await?;

    let role = Role::from_str(&user.role)?;
    let token = generate_token(user.id.to_string(), role)?;

    txn.commit().await?;

    Ok(AuthResponse {
        token,
        user: User {
            username: user.username,
            email: user.email,
        },
    })
}

pub async fn login(input: LoginInput, db: &DatabaseConnection) -> Result<AuthResponse, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(input.email))
        .one(db)
        .await?;

    let user = match user {
        Some(u) => u,
        None => return Err(AppError::InvalidCredentials.into()),
    };

    verify_password(&input.password, &user.password)?;

    let role = Role::from_str(&user.role)?;
    let token = generate_token(user.id.to_string(), role)?;

    Ok(AuthResponse {
        token,
        user: User {
            username: user.username,
            email: user.email,
        },
    })
}

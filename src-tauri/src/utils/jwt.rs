use crate::enums::role::Role;
use crate::errors::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: String, duration_minutes: i64, role: Role) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::minutes(duration_minutes)).timestamp();
        Self {
            sub: user_id,
            role,
            exp,
            iat: now.timestamp(),
        }
    }
}

fn get_jwt_secret() -> String {
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| "development-secret-key-change-in-production".to_string())
}

fn get_access_token_duration() -> i64 {
    env::var("ACCESS_TOKEN_DURATION_MINUTES")
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(1)
}

pub fn generate_token(user_id: String, role: Role) -> Result<String, AppError> {
    let duration = get_access_token_duration();
    let claims = Claims::new(user_id, duration * 24 * 60, role);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_ref()),
    )
    .map_err(|e| {
        eprintln!("Failed to generate access token: {:?}", e);
        AppError::Jwt(e)
    })
}

pub fn validate_token(token: &str) -> Result<Claims, AppError> {
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_ref()),
        &validation,
    )
    .map_err(|e| {
        eprintln!("Token validation failed: {:?}", e);
        AppError::Jwt(e)
    })?;

    let now = Utc::now().timestamp();
    if token_data.claims.exp < now {
        return Err(AppError::Jwt(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::ExpiredSignature,
        )));
    }

    Ok(token_data.claims)
}

pub fn extract_user_id_from_token(token: &str) -> Result<String, AppError> {
    let claims = validate_token(token)?;
    Ok(claims.sub)
}

pub fn validate_token_with_role(token: &str, required_role: Role) -> Result<Claims, AppError> {
    let claims = validate_token(token)?;

    if claims.role != required_role {
        return Err(AppError::Unauthorized);
    }

    Ok(claims)
}

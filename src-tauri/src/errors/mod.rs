use serde::Serialize;
use thiserror::Error;
use ts_rs::TS;
use uuid::Error as UuidError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Password hashing error")]
    Hash,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid UUID")]
    InvalidUUID,

    #[error("{0} not found")]
    NotFound(String),

    #[error("Forbidden")]
    Forbidden,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Duplicate entry: {0}")]
    Duplicate(String),

    #[error("Other error: {0}")]
    Other(String),
}

#[derive(TS, Serialize, Clone)]
#[ts(export, export_to = "../../types/ErrorResponse.d.ts")]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl From<AppError> for ErrorResponse {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Database(ref e) => {
                eprintln!("Database error: {:?}", e);

                if let sea_orm::DbErr::Query(sea_orm::RuntimeErr::SqlxError(sqlx_err)) = e {
                    if let Some(db_err) = sqlx_err.as_database_error() {
                        if db_err.code().as_deref() == Some("23505") {
                            return ErrorResponse {
                                code: "DUPLICATE_ENTRY".to_string(),
                                message: "This record already exists".to_string(),
                                details: Some("Email or username already registered".to_string()),
                            };
                        }
                    }
                }

                ErrorResponse {
                    code: "DATABASE_ERROR".to_string(),
                    message: "Database operation failed".to_string(),
                    details: Some(e.to_string()),
                }
            }

            AppError::Jwt(_) => ErrorResponse {
                code: "INVALID_TOKEN".to_string(),
                message: "Invalid or expired token".to_string(),
                details: None,
            },

            AppError::Hash => ErrorResponse {
                code: "HASH_ERROR".to_string(),
                message: "Password processing error".to_string(),
                details: None,
            },

            AppError::InvalidCredentials => ErrorResponse {
                code: "INVALID_CREDENTIALS".to_string(),
                message: "Invalid email or password".to_string(),
                details: None,
            },

            AppError::Unauthorized => ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            },

            AppError::Forbidden => ErrorResponse {
                code: "FORBIDDEN".to_string(),
                message: "You do not have permission to perform this action".to_string(),
                details: None,
            },

            AppError::InvalidUUID => ErrorResponse {
                code: "INVALID_UUID".to_string(),
                message: "Invalid UUID format".to_string(),
                details: Some(
                    "UUID must be in format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string(),
                ),
            },

            AppError::NotFound(resource) => ErrorResponse {
                code: "NOT_FOUND".to_string(),
                message: format!("{} not found", resource),
                details: None,
            },

            AppError::Validation(msg) => ErrorResponse {
                code: "VALIDATION_ERROR".to_string(),
                message: msg,
                details: None,
            },

            AppError::Duplicate(field) => ErrorResponse {
                code: "DUPLICATE_ENTRY".to_string(),
                message: format!("{} already exists", field),
                details: None,
            },

            AppError::Other(msg) => ErrorResponse {
                code: "INTERNAL_ERROR".to_string(),
                message: "An unexpected error occurred".to_string(),
                details: Some(msg),
            },
        }
    }
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        let response: ErrorResponse = error.into();
        serde_json::to_string(&response).unwrap_or_else(|_| {
            r#"{"code":"INTERNAL_ERROR","message":"Failed to serialize error"}"#.to_string()
        })
    }
}

impl AppError {
    pub fn not_found(resource: &str) -> Self {
        AppError::NotFound(resource.to_string())
    }

    pub fn validation(message: String) -> Self {
        AppError::Validation(message)
    }

    pub fn duplicate(field: &str) -> Self {
        AppError::Duplicate(field.to_string())
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(_: argon2::password_hash::Error) -> Self {
        AppError::Hash
    }
}

impl From<UuidError> for AppError {
    fn from(_: UuidError) -> Self {
        AppError::InvalidUUID
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Other(format!("JSON error: {}", e))
    }
}

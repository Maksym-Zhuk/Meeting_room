use validator::Validate;
use validator::ValidationError;

use crate::errors::AppError;

pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
    if uuid::Uuid::parse_str(uuid).is_ok() {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_uuid"))
    }
}

pub fn validate_input<T: Validate>(input: &T) -> Result<(), AppError> {
    input
        .validate()
        .map_err(|e| AppError::validation(e.to_string()))
}

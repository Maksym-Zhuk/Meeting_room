use validator::ValidationError;

pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
    if uuid::Uuid::parse_str(uuid).is_ok() {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_uuid"))
    }
}

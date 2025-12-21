use serde::Deserialize;
use ts_rs::TS;
use validator::Validate;

#[derive(TS, Debug, Deserialize, Validate)]
#[ts(export, export_to = "../../types/input/RegisterInput.d.ts")]
pub struct RegisterInput {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

use serde::Deserialize;
use ts_rs::TS;
use validator::Validate;

#[derive(TS, Debug, Deserialize, Validate)]
#[ts(export, export_to = "../../types/inputs/LoginInput.d.ts")]
pub struct LoginInput {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

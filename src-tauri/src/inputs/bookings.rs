use serde::Deserialize;
use ts_rs::TS;
use validator::Validate;

use crate::utils::validator::validate_uuid;

#[derive(TS, Debug, Deserialize, Validate)]
#[ts(export, export_to = "../../types/inputs/CreateBookingInput.d.ts")]
pub struct CreateBookingInput {
    #[validate(custom(function = "validate_uuid"))]
    pub room_id: String,

    pub start_at: i64,

    pub end_at: i64,

    #[validate(length(min = 1, max = 255))]
    pub title: String,
}

#[derive(TS, Debug, Deserialize, Validate)]
#[ts(export, export_to = "../../types/inputs/UpdateBookingInput.d.ts")]
pub struct UpdateBookingInput {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,

    pub start_at: Option<i64>,

    pub end_at: Option<i64>,

    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
}

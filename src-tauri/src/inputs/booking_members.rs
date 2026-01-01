use serde::Deserialize;
use ts_rs::TS;
use validator::Validate;

use crate::utils::validator::validate_uuid;

#[derive(TS, Debug, Deserialize, Validate)]
#[ts(export, export_to = "../../types/inputs/CreateBookingMemberInput.d.ts")]
pub struct CreateBookingMemberInput {
    #[validate(custom(function = "validate_uuid"))]
    pub booking_id: String,

    pub email: String,
}

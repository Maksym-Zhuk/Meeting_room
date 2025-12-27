use crate::utils::validator::validate_uuid;
use serde::Deserialize;
use ts_rs::TS;
use validator::Validate;

#[derive(TS, Debug, Deserialize, Validate)]
#[ts(export, export_to = "../../types/inputs/CreateOrganizationInput.d.ts")]
pub struct CreateOrganizationInput {
    #[validate(length(max = 255))]
    pub name: String,
}

#[derive(TS, Debug, Deserialize, Validate)]
#[ts(export, export_to = "../../types/inputs/UpdateOrganizationInput.d.ts")]
pub struct UpdateOrganizationInput {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,

    #[validate(length(max = 255))]
    pub name: String,
}

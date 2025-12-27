use serde::Deserialize;
use ts_rs::TS;
use validator::{Validate, ValidationError};

use crate::{enums::organization_roles::OrganizationRole, utils::validator::validate_uuid};

fn validate_role(role: &OrganizationRole) -> Result<(), ValidationError> {
    match role {
        OrganizationRole::Admin | OrganizationRole::Member | OrganizationRole::Owner => Ok(()),
        _ => Err(ValidationError::new("invalid_role")),
    }
}

#[derive(TS, Debug, Deserialize, Validate)]
#[ts(
    export,
    export_to = "../../types/inputs/CreateOrganizationMemberInput.d.ts"
)]
pub struct CreateOrganizationMemberInput {
    #[validate(custom(function = "validate_uuid"))]
    pub organization_id: String,

    #[validate(custom(function = "validate_role"))]
    pub role: OrganizationRole,
}

#[derive(TS, Debug, Deserialize, Validate)]
#[ts(
    export,
    export_to = "../../types/inputs/UpdateOrganizationMemberInput.d.ts"
)]
pub struct UpdateOrganizationMemberInput {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,

    #[validate(custom(function = "validate_role"))]
    pub role: OrganizationRole,
}

use serde::Serialize;
use ts_rs::TS;

use crate::enums::organization_roles::OrganizationRole;

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "../../types/responses/OrganizationMembers.d.ts")]
pub struct OrganizationMembers {
    pub id: String,
    pub organization_id: String,
    pub user_id: String,
    pub role: OrganizationRole,
}

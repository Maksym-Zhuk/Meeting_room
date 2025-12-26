use std::str::FromStr;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::errors::AppError;

#[derive(TS, Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[ts(export, export_to = "../../types/enums/OrganizationRole.d.ts")]
#[serde(rename_all = "lowercase")]
pub enum OrganizationRole {
    Owner,
    Admin,
    Member,
}

impl FromStr for OrganizationRole {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "owner" => Ok(OrganizationRole::Owner),
            "admin" => Ok(OrganizationRole::Admin),
            "member" => Ok(OrganizationRole::Member),
            _ => Err(AppError::validation("invalid role".into())),
        }
    }
}

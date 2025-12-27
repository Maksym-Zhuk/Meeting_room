use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::errors::AppError;

#[derive(TS, Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[ts(export, export_to = "../../types/enums/Role.d.ts")]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
}

impl FromStr for Role {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            _ => Err(AppError::validation("invalid role".into())),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::User => write!(f, "user"),
        }
    }
}

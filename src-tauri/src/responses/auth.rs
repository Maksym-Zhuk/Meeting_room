use serde::Serialize;
use ts_rs::TS;

use crate::responses::users::User;

#[derive(TS, Serialize, Clone)]
#[ts(export, export_to = "../../types/responses/AuthResponse.d.ts")]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

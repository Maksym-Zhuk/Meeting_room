use serde::Serialize;

use crate::models::user::User;

#[derive(Serialize, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

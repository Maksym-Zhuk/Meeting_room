use serde::Serialize;

use crate::responses::users::User;

#[derive(Serialize, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

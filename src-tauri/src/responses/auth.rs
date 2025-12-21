use serde::Serialize;

use crate::entity::user::Model as User;

#[derive(Serialize, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

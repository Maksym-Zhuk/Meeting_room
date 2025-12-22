use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Serialize, Clone)]
#[ts(export, export_to = "../../types/models/User.d.ts")]
pub struct User {
    pub email: String,
    pub username: String,
}

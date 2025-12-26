use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Serialize, Clone)]
#[ts(export, export_to = "../../types/responses/User.d.ts")]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
}

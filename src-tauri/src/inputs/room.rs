use serde::Deserialize;
use ts_rs::TS;
use validator::Validate;

#[derive(TS, Deserialize, Debug, Validate)]
#[ts(export, export_to = "../../types/inputs/CreateRoomInput.d.ts")]
pub struct CreateRoomInput {
    pub title: String,
    pub start: i64,
    pub end: i64,
    pub members: Vec<String>,
    pub admins: Vec<String>,
}

#[derive(TS, Deserialize, Debug, Validate)]
#[ts(export, export_to = "../../types/inputs/UpdateRoomInput.d.ts")]
pub struct UpdateRoomInput {
    pub id: String,
    pub title: Option<String>,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub members: Option<Vec<String>>,
    pub admins: Option<Vec<String>>,
}

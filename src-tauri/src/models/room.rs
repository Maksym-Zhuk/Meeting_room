use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "../../types/models/Room.d.ts")]
pub struct Room {
    pub id: String,
    pub title: String,
    pub start: i64,
    pub end: i64,
    pub creator: String,
    pub members: Vec<String>,
    pub admins: Vec<String>,
}

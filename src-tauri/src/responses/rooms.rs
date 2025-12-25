use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "../../types/models/Room.d.ts")]
pub struct Room {
    pub id: String,
    pub name: String,
    pub organization_id: String,
}

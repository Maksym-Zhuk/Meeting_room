use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "../../types/responses/Room.d.ts")]
pub struct Room {
    pub id: String,
    pub name: String,
    pub organization_id: String,
    pub created_by: String,
}

use serde::Deserialize;
use ts_rs::TS;
use validator::Validate;

#[derive(TS, Deserialize, Debug, Validate)]
#[ts(export, export_to = "../../types/inputs/CreateRoomInput.d.ts")]
pub struct CreateRoomInput {
    pub name: String,
    pub organization_id: String,
}

#[derive(TS, Deserialize, Debug, Validate)]
#[ts(export, export_to = "../../types/inputs/UpdateRoomInput.d.ts")]
pub struct UpdateRoomInput {
    pub id: String,
    pub name: Option<String>,
}

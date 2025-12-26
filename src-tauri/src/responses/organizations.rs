use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "../../types/responses/Organization.d.ts")]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub creator_id: String,
}

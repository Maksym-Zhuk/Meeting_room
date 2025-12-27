use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "../../types/responses/Response.d.ts")]
pub struct Response {
    pub message: String,
}

use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "../../types/responses/Booking.d.ts")]
pub struct Booking {
    pub id: String,
    pub room_id: String,
    pub title: String,
    pub created_by: String,
    pub start_at: i64,
    pub end_at: i64,
}

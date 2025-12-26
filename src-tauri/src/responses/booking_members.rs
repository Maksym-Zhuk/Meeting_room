use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "../../types/responses/BookingMembers.d.ts")]
pub struct BookingMembers {
    pub id: String,
    pub booking_id: String,
    pub user_id: String,
}

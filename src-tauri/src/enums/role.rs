use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum Role {
    Admin,
    User,
}

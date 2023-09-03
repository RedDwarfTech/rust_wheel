use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RdUserInfo {
    pub id: i64,
    pub nickname: String,
}
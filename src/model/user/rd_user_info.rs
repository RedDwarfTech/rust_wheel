use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RdUserInfo {
    pub id: i64, // https://stackoverflow.com/questions/77032887/is-it-possible-try-to-convert-the-data-type-when-using-rust-serde-json
    pub nickname: String,
    pub device_id: String,
    pub app_id: String
}
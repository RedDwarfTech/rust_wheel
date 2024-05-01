use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i64,
    pub device_id: String,
    pub app_id: String,
    pub exp: i64,
    pub pin: i32,
}
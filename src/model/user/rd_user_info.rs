use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RdUserInfo {
    #[serde(rename = "userId", default)]
    pub id: i64, // https://stackoverflow.com/questions/77032887/is-it-possible-try-to-convert-the-data-type-when-using-rust-serde-json
    pub nickname: String,
    #[serde(rename = "deviceId", default)]
    pub device_id: String,
    #[serde(rename = "appId", default)]
    pub app_id: String,
    #[serde(rename = "avatarUrl", default)]
    pub avatar_url: String,
    #[serde(rename = "autoRenewProductExpireTimeMs", default)]
    pub auto_renew_product_expire_time_ms: i64,
    #[serde(rename = "appName", default)]
    pub app_name: String,
}
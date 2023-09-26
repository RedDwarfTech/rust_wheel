use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RdUserInfo {
    pub id: String, // https://stackoverflow.com/questions/77032887/is-it-possible-try-to-convert-the-data-type-when-using-rust-serde-json
    pub nickname: String,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: i32,
    device_id: String,
    app_id: String,
    exp: usize,
    pin: i32,
}
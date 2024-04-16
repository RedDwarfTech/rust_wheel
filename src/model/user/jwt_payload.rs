use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct JwtPayload {
    pub userId: i64, 
    pub deviceId: String,
    pub appId: String,
    pub lt: i32,
    pub et: i64,
    pub pid: i32
}
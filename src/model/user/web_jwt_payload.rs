use serde::{Serialize, Deserialize};

/**
 * this jwt payload used for the customer client
 * maybe different with the bizinesse client
 */
#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct WebJwtPayload {
    pub userId: i64, 
    pub deviceId: String,
    pub appId: String,
    pub lt: i32,
    pub et: i64,
    pub pid: i32,
    pub exp: usize,
}
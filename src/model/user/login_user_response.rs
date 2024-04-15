use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, Clone)]
#[allow(non_snake_case)]
pub struct LoginUserResponse {
    pub nickname: String,
    pub userId: i64,
    pub appName: String,
    pub avatarUrl: String,
    pub autoRenewProductExpireTimeMs: i64
}
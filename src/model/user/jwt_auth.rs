use core::fmt;
use std::future::{ready, Ready};
use super::login_user_info::LoginUserInfo;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{FromRequest, HttpRequest};
use reqwest::header::{HeaderValue, ToStrError};
use serde::Serialize;
use serde_json::from_str;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

fn get_header_value(header_value: &HeaderValue) -> Result<&str, ToStrError> {
    let value_str = header_value.to_str();
    return value_str;
}

impl FromRequest for LoginUserInfo {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req.headers().get("x-access-token");
        let x_request_id = req.headers().get("x-request-id");
        match token {
            Some(value) => {
                let t = get_header_value(value).unwrap();
                let parts: Vec<&str> = t.split(".").collect();
                let payload_base64 = parts[1];
                let payload_str = base64::decode(payload_base64).unwrap();
                let payload_json = from_str::<serde_json::Value>(&String::from_utf8(payload_str).unwrap()).unwrap();
                let payload_claims = payload_json.as_object().unwrap();
                let user_id = payload_claims.get("userId");
                let app_id = payload_claims.get("appId");
                let device_id = payload_claims.get("deviceId");
                let vip_expire_time = payload_claims.get("et");
                let login_user_info = LoginUserInfo {
                    token: t.to_string(),
                    userId: user_id.unwrap().as_i64().unwrap(),
                    appId: app_id.unwrap().to_string(),
                    xRequestId: get_header_value(x_request_id.unwrap()).unwrap().to_string(),
                    deviceId: device_id.unwrap().to_string(),
                    vipExpireTime: vip_expire_time.unwrap().as_i64().unwrap_or_default()
                };
                ready(Ok(login_user_info))
            }
            None => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "the user belonging to this token no logger exists".to_string(),
                };
                ready(Err(ErrorUnauthorized(json_error)))
            }
        }
    }
}
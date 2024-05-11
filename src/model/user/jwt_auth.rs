use crate::common::error::jwt_token_error::JwtTokenError;

use super::login_user_info::LoginUserInfo;
use super::web_jwt_payload::WebJwtPayload;
use actix_web::error::ErrorUnauthorized;
use actix_web::web::Query;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{FromRequest, HttpRequest};
use core::fmt;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header};
use log::error;
use reqwest::header::{HeaderValue, ToStrError};
use serde::Serialize;
use serde_json::from_str;
use std::collections::HashMap;
use std::env;
use std::future::{ready, Ready};
use uuid::Uuid;

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

fn get_auth_header(req: &HttpRequest) -> Option<String> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_value) = auth_header.to_str() {
            if header_value.starts_with("Bearer ") {
                let token = header_value.trim_start_matches("Bearer ");
                return Some(token.to_string());
            }
        }
    }
    return None;
}

fn get_params_access_token(request: &HttpRequest) -> Option<String> {
    let q_str = request.query_string();
    if q_str.is_empty() {
        return None;
    }
    let params = Query::<HashMap<String, String>>::from_query(request.query_string()).unwrap();
    let access_token = params.get("access_token");
    return access_token.map(|s| s.to_owned());
}

pub fn get_forward_url_path(request: &HttpRequest) -> Option<&str>{
    let x_header = request.headers().get("X-Forwarded-Uri");
    if x_header.is_none() {
        return None;
    }
    let x_header_str = x_header.unwrap().to_str();
    if let Err(e) = x_header_str {
        error!("url get header str failed: {}", e);
        return None;
    }
    if x_header_str.as_ref().unwrap().is_empty() {
        return None;
    }
    let key_value_pairs: Vec<&str> = x_header_str.unwrap().split('?').collect();
    return key_value_pairs.get(0).copied();
}

fn get_forward_params_access_token(request: &HttpRequest) -> Option<String> {
    let x_header = request.headers().get("X-Forwarded-Uri");
    if x_header.is_none() {
        return None;
    }
    let x_header_str = x_header.unwrap().to_str();
    if let Err(e) = x_header_str {
        error!("get header str failed: {}", e);
        return None;
    }
    if x_header_str.as_ref().unwrap().is_empty() {
        return None;
    }
    let key_value_pairs: Vec<&str> = x_header_str.unwrap().split('?').collect();
    let pairs = key_value_pairs.get(1);
    if pairs.is_none() {
        return None;
    }
    let query_pairs: Vec<&str> = pairs.unwrap().split('&').collect();
    for pair in query_pairs {
        if pair.contains("access_token=") {
            let access_token: Vec<&str> = pair.split('=').collect();
            if let Some(token) = access_token.get(1) {
                return Some(token.to_string());
            }
        }
    }
    return None;
}

/// get token from the http standard Authorization by default
/// if failed, get the token from http query parameter 'access_token'
pub fn get_auth_token(req: &HttpRequest) -> String {
    let mut token = get_auth_header(req);
    if token.is_none() {
        token = get_params_access_token(req);
    }
    return token.unwrap_or_default();
}

// https://community.traefik.io/t/is-it-possible-to-forward-the-query-parameters-when-authforward/19926/1
// https://stackoverflow.com/questions/77154811/how-to-forward-the-url-query-parameter-access-token-when-using-auth-forward-in-t
pub fn get_auth_token_from_traefik(req: &HttpRequest) -> String {
    let mut token = get_auth_header(req);
    if token.is_none() {
        token = get_forward_params_access_token(req);
    }
    return token.unwrap_or_default();
}

pub fn create_access_token(jwt_payload: &WebJwtPayload) -> String {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = &EncodingKey::from_secret(jwt_secret.as_ref());
    let token = encode(&Header::default(), &jwt_payload, key);
    return token.unwrap();
}

pub fn verify_jwt_token(token: &str) -> Option<ErrorKind> {
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
    match decode::<serde_json::Value>(token, &decoding_key, &Default::default()) {
        Ok(token_data) => {
            if let Some(exp) = token_data.claims.get("exp") {
                let current_time = chrono::Utc::now().timestamp();
                let exp_time1 = exp.as_i64().unwrap();
                if exp_time1 < current_time {
                    return Some(ErrorKind::ExpiredSignature);
                }
            }
            None
        }
        Err(err) => {
            return Some(err.kind().clone())
        },
    }
}

impl FromRequest for LoginUserInfo {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = get_auth_token(req);
        let x_request_id = req.headers().get("x-request-id");
        if !token.is_empty() {
            let parts: Vec<&str> = token.split(".").collect();
            let payload_base64 = parts[1];
            let payload_str = base64::decode(payload_base64).unwrap();
            let payload_json =
                from_str::<serde_json::Value>(&String::from_utf8(payload_str).unwrap()).unwrap();
            let payload_claims = payload_json.as_object().unwrap();
            let user_id = payload_claims.get("userId");
            let app_id = payload_claims.get("appId");
            let device_id = payload_claims.get("deviceId");
            let vip_expire_time = payload_claims.get("et");
            let x_request_id_value = if x_request_id.is_some() {
                get_header_value(x_request_id.unwrap()).unwrap().to_string()
            } else {
                let uuid = Uuid::new_v4();
                uuid.to_string()
            };
            let login_user_info = LoginUserInfo {
                token: token.to_string(),
                userId: user_id.unwrap().as_i64().unwrap(),
                // https://stackoverflow.com/questions/72345657/how-do-i-get-the-string-value-of-a-json-value-without-quotes
                appId: app_id
                    .unwrap()
                    .as_str()
                    .expect("get app id failed")
                    .to_string(),
                xRequestId: x_request_id_value,
                deviceId: device_id
                    .unwrap()
                    .as_str()
                    .expect("get device id failed")
                    .to_string(),
                vipExpireTime: vip_expire_time.unwrap().as_i64().unwrap_or_default(),
            };
            ready(Ok(login_user_info))
        } else {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: "the user belonging to this token no logger exists".to_string(),
            };
            ready(Err(ErrorUnauthorized(json_error)))
        }
    }
}

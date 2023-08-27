use actix_web::{Responder, HttpResponse};
use serde::Serialize;
use crate::model::response::api_response::ApiResponse;

/// sometimes we need to keep the response type for docs purpose
/// https://github.com/GREsau/okapi/issues/102
pub fn box_type_rest_response<T>(data: T) -> ApiResponse<T> where T: Serialize + Default {
    let res = ApiResponse {
        result: data,
        ..Default::default()
    };
    return res;
}

// https://stackoverflow.com/questions/76987201/distinct-uses-of-impl-trait-result-in-different-opaque-types-in-rust
pub fn box_actix_rest_response<T>(data: T) -> HttpResponse where T: Serialize + Default{
    let res = ApiResponse {
        result: data,
        ..Default::default()
    };
    HttpResponse::Ok().json(res)
}

pub fn box_error_actix_rest_response <T>(data: T, result_code: String, msg: String) -> HttpResponse where T: Serialize + Default {
    let res = ApiResponse {
        result: data,
        statusCode: "200".to_string(),
        resultCode: result_code,
        msg
    };
    HttpResponse::Ok().json(res)
}

/// sometimes we need to keep the response type for docs purpose
/// https://github.com/GREsau/okapi/issues/102
pub fn box_error_type_rest_response <T>(data: T, result_code: String, msg: String) -> ApiResponse<T> where T: Serialize + Default {
    let res = ApiResponse {
        result: data,
        statusCode: "200".to_string(),
        resultCode: result_code,
        msg
    };
    return res;
}
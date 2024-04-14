use crate::model::{error::custom_error::CustomError, response::api_response::ApiResponse};
use actix_web::HttpResponse;
use serde::Serialize;

/// sometimes we need to keep the response type for docs purpose
/// https://github.com/GREsau/okapi/issues/102
pub fn box_type_rest_response<T>(data: T) -> ApiResponse<T>
where
    T: Serialize + Default,
{
    let res = ApiResponse {
        result: data,
        ..Default::default()
    };
    return res;
}

// https://stackoverflow.com/questions/76987201/distinct-uses-of-impl-trait-result-in-different-opaque-types-in-rust
pub fn box_actix_rest_response<T>(data: T) -> HttpResponse
where
    T: Serialize + Default,
{
    let res = ApiResponse {
        result: data,
        ..Default::default()
    };
    HttpResponse::Ok().json(res)
}

pub fn box_error_actix_rest_response<T>(data: T, result_code: String, msg: String) -> HttpResponse
where
    T: Serialize + Default,
{
    let res = ApiResponse {
        result: data,
        statusCode: "200".to_string(),
        resultCode: result_code,
        msg,
    };
    HttpResponse::Ok().json(res)
}

/// sometimes we need to keep the response type for docs purpose
/// https://github.com/GREsau/okapi/issues/102
pub fn box_error_type_rest_response<T>(data: T, result_code: String, msg: String) -> ApiResponse<T>
where
    T: Serialize + Default,
{
    let res = ApiResponse {
        result: data,
        statusCode: "200".to_string(),
        resultCode: result_code,
        msg,
    };
    return res;
}

pub fn box_error_response<T>(data: T, err: CustomError) -> ApiResponse<T>
where
    T: Serialize + Default,
{
    let res = ApiResponse {
        result: data,
        statusCode: "200".to_string(),
        resultCode: "code from err".to_owned(),
        msg: "msg from err".to_owned(),
    };
    return res;
}

use rocket::response::content;
use rocket::serde::json::serde_json;
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

pub fn box_rest_response<T>(data: T) -> content::RawJson<String> where T: Serialize + Default {
    let res = ApiResponse {
        result: data,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::RawJson(response_json);
}

pub fn box_error_rest_response <T>(data: T, result_code: String, msg: String) -> content::RawJson<String> where T: Serialize + Default {
    let res = ApiResponse {
        result: data,
        statusCode: "200".to_string(),
        resultCode: result_code,
        msg
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::RawJson(response_json);
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
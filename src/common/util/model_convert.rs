use diesel::QueryResult;
use rocket::response::content;
use rocket::serde::json::serde_json;
use serde::Serialize;
use crate::model::response::api_response::ApiResponse;
use crate::model::response::pagination::Pagination;
use crate::model::response::pagination_response::PaginationResponse;

pub fn map_pagination_res<U>(result: QueryResult<(Vec<U>, i64, i64)>, page_num: i64,page_size: i64) -> PaginationResponse<Vec<U>>{
    let page_result = Pagination{
        pageNum: page_num,
        pageSize: page_size,
        total: result.as_ref().unwrap().2
    };
    let resp = PaginationResponse{
        pagination: page_result,
        list: result.unwrap().0
    };
    return resp;
}

pub fn map_pagination_from_list<U>(list: Vec<U>, page_num: i64,page_size: i64, total: i64) -> PaginationResponse<Vec<U>>{
    let page_result = Pagination{
        pageNum: page_num,
        pageSize: page_size,
        total
    };
    let resp = PaginationResponse{
        pagination: page_result,
        list
    };
    return resp;
}

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

/// https://stackoverflow.com/questions/72569425/borrowed-value-does-not-live-long-enough-when-write-an-generic-object-mapping-fu
pub fn map_entity<T,E>(sources: Vec<T>) -> Vec<E> where for<'a> E: From<&'a T>{
    sources.iter().map(E::from).collect()
}


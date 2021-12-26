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

pub fn box_rest_response<T>(data: T) -> content::Json<String> where T: Serialize + Default {
    let res = ApiResponse {
        result: data,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}

pub fn box_error_rest_response <T>(data: T, resultCode: String) -> content::Json<String> where T: Serialize + Default {
    let res = ApiResponse {
        result: data,
        statusCode: "200".to_string(),
        resultCode
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return content::Json(response_json);
}
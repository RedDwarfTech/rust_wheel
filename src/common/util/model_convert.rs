use diesel::QueryResult;
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

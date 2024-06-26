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
        data: result.unwrap().0
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
        data: list
    };
    return resp;
}


/// https://stackoverflow.com/questions/72569425/borrowed-value-does-not-live-long-enough-when-write-an-generic-object-mapping-fu
pub fn map_entity<T,E>(sources: Vec<T>) -> Vec<E> where for<'a> E: From<&'a T>{
    sources.iter().map(E::from).collect()
}


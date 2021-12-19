use diesel::QueryResult;
use crate::common::query::pagination::{PaginateForQueryFragment, PaginateForQuerySource};
use crate::model::response::pagination::Pagination;
use crate::model::response::pagination_response::PaginationResponse;

pub fn map_pagination_res<U>(result: QueryResult<(Vec<U>, i64)>) -> PaginationResponse<U>{



    let page_result = Pagination{
        pageNum: result.paginate().page,
        pageSize: result.paginate().per_page,
        total: 20
    };
    let resp = PaginationResponse{
        pagination: page_result,
        list: result.0
    };
    return resp;
}



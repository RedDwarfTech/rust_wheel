use crate::model::response::pagination::Pagination;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PaginationResponse<T> {
    pub pagination: Pagination,
    pub list: T
}



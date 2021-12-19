use crate::model::response::pagination::Pagination;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize,Default)]
#[allow(non_snake_case)]
pub struct PaginationResponse<T> {
    pub pagination: Pagination,
    pub list: T
}



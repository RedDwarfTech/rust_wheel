use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
#[allow(non_snake_case)]
pub struct Pagination {
    pub pageNum: i64,
    pub pageSize: i64,
    pub total: i64
}



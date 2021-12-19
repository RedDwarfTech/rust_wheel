
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Pagination {
    pub pageNum: i64,
    pub pageSize: i64,
    pub total: i64
}



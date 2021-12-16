
use diesel::prelude::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::query_builder::{QueryFragment, Query, AstPass};
use diesel::pg::Pg;
use diesel::sql_types::BigInt;
use diesel::QueryId;

pub trait PaginateForQueryFragment: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
    is_sub_query: bool
}



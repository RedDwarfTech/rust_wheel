use diesel::{query_builder::QueryFragment, pg::Pg};
use super::pagination::Paginated;

const DEFAULT_PER_PAGE: i64 = 10;

pub trait PaginateForQueryFragment: Sized {
    fn paginate(self, page: i64, is_big_table: bool) -> Paginated<Self>;
}

impl<T> PaginateForQueryFragment for T
    where T: QueryFragment<Pg>{
    fn paginate(self, page: i64, is_big_table: bool) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: 10,
            page,
            is_sub_query: true,
            is_big_table,
            offset: (page - 1) * DEFAULT_PER_PAGE,
        }
    }
}
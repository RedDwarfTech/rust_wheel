
use diesel::prelude::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::query_builder::{QueryFragment, Query, AstPass};
use diesel::pg::Pg;
use diesel::sql_types::BigInt;
use diesel::QueryId;
use serde::{Serialize, Deserialize};
use crate::common::query::page_query_handler::{ handle_table_query };

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
            is_big_table
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId, Serialize, Deserialize, Default)]
pub struct Paginated<T> {
    pub query: T,
    pub page: i64,
    pub per_page: i64,
    pub is_sub_query: bool,
    pub is_big_table: bool
}

impl<T> Paginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated { per_page, ..self }
    }

    pub fn load_and_count_pages<U>(self, conn: &PgConnection) -> QueryResult<(Vec<U>, i64)>
        where
            Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total_pages))
    }

    pub fn load_and_count_pages_total<U>(self, conn: &PgConnection) -> QueryResult<(Vec<U>, i64, i64)>
        where
            Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total_pages,total))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}


impl<T> QueryFragment<Pg> for Paginated<T>
    where
        T: QueryFragment<Pg>,
{
    fn walk_ast(&self, out: AstPass<Pg>) -> QueryResult<()> {
        handle_table_query(&self, out).expect("TODO: panic message");
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct QuerySourceToQueryFragment<T> {
    query_source: T,
}

impl<FC, T> QueryFragment<Pg> for QuerySourceToQueryFragment<T>
    where
        FC: QueryFragment<Pg>,
        T: QuerySource<FromClause=FC>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        self.query_source.from_clause().walk_ast(out.reborrow())?;
        Ok(())
    }
}

pub trait PaginateForQuerySource: Sized {
    fn paginate(self, page: i64, is_big_table: bool) -> Paginated<QuerySourceToQueryFragment<Self>>;
}

impl<T> PaginateForQuerySource for T
    where T: QuerySource {
    fn paginate(self, page: i64, is_big_table: bool) -> Paginated<QuerySourceToQueryFragment<Self>> {
        Paginated {
            query: QuerySourceToQueryFragment {query_source: self},
            per_page: 10,
            page,
            is_sub_query: false,
            is_big_table
        }
    }
}
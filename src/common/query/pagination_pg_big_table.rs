
use diesel::prelude::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::query_builder::{QueryFragment, Query};
use diesel::pg::Pg;
use diesel::sql_types::BigInt;
use diesel::QueryId;
use serde::{Serialize, Deserialize};

pub trait PaginateForPgBigTableQueryFragment: Sized {
    fn paginate_pg_big_table(self, page: i64, table_name: String) -> PgBigTablePaginated<Self>;
}

impl<T> PaginateForPgBigTableQueryFragment for T
    where T: QueryFragment<Pg>{
    fn paginate_pg_big_table(self, page: i64, table_name: String) -> PgBigTablePaginated<Self> {
        PgBigTablePaginated {
            query: self,
            per_page: 10,
            page,
            is_sub_query: true,
            table_name
        }
    }
}

#[derive(Debug, Clone, QueryId, Serialize, Deserialize, Default)]
pub struct PgBigTablePaginated<T> {
    pub query: T,
    pub page: i64,
    pub per_page: i64,
    pub is_sub_query: bool,
    pub table_name: String
}

impl<T> PgBigTablePaginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        PgBigTablePaginated { per_page, ..self }
    }

    pub fn pg_big_table_load_and_count_pages<U>(self,conn: &mut PgConnection) -> QueryResult<(Vec<U>, i64)>
        where
            Self: for<'a> LoadQuery<'a, PgConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total_pages))
    }

    pub fn pg_big_table_load_and_count_pages_total<U>(self, conn: &mut PgConnection) -> QueryResult<(Vec<U>, i64, i64)>
        where
            Self: for<'a> LoadQuery<'a, PgConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total_pages,total))
    }
}

impl<T: Query> Query for PgBigTablePaginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for PgBigTablePaginated<T> {}

/** 
impl<T> QueryFragment<Pg> for PgBigTablePaginated<T>
    where
        T: QueryFragment<Pg>,
{
    fn walk_ast(&self, out: AstPass<Pg>) -> QueryResult<()> {
        handle_big_table_query(&self, out).expect("TODO: panic message");
        Ok(())
    }
}
**/

#[derive(Debug, Clone, Copy, QueryId)]
pub struct PgBigTableQuerySourceToQueryFragment<T> {
    query_source: T,
}

/** 
impl<FC, T> QueryFragment<Pg> for PgBigTableQuerySourceToQueryFragment<T>
    where
        FC: QueryFragment<Pg>,
        T: QuerySource<FromClause=FC>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        self.query_source.from_clause().walk_ast(out.reborrow())?;
        Ok(())
    }
}
**/

pub trait PaginateForPgBigTableQuerySource: Sized {
    fn paginate_pg_big_table(self, page: i64, table_name: String) -> PgBigTablePaginated<PgBigTableQuerySourceToQueryFragment<Self>>;
}

impl<T> PaginateForPgBigTableQuerySource for T
    where T: QuerySource {
    fn paginate_pg_big_table(self, page: i64, table_name: String) -> PgBigTablePaginated<PgBigTableQuerySourceToQueryFragment<Self>> {
        PgBigTablePaginated {
            query: PgBigTableQuerySourceToQueryFragment {query_source: self},
            per_page: 10,
            page,
            is_sub_query: false,
            table_name
        }
    }
}
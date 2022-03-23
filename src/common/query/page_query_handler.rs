use diesel::pg::Pg;
use diesel::query_builder::{AstPass, QueryFragment};
use diesel::QueryResult;
use diesel::sql_types::BigInt;
use crate::common::query::pagination::Paginated;
use crate::common::query::pagination_pg_big_table::PgBigTablePaginated;

pub fn handle_table_query<T: QueryFragment<Pg>>(this: &Paginated<T>, mut out: AstPass<Pg>) -> QueryResult<()> {
    // https://stackoverflow.com/questions/6218902/the-sql-over-clause-when-and-why-is-it-useful
    out.push_sql("SELECT *, COUNT(*) OVER () FROM ");
    if this.is_sub_query {
        out.push_sql("(");
    }
    this.query.walk_ast(out.reborrow())?;
    if this.is_sub_query {
        out.push_sql(")");
    }
    out.push_sql(" t LIMIT ");
    out.push_bind_param::<BigInt, _>(&this.per_page)?;
    out.push_sql(" OFFSET ");
    let offset = (this.page - 1) * this.per_page;
    out.push_bind_param::<BigInt, _>(&offset)?;
    Ok(())
}

///
/// the key to speed up the PostgreSQL big table query was:
/// 1. use the estimate rows count
/// 2. use cursor to optimize the end of pages speed
///
///
pub fn handle_big_table_query<T: QueryFragment<Pg>>(this: &PgBigTablePaginated<T>, mut out: AstPass<Pg>)-> QueryResult<()>{
    // https://www.sqlstyle.guide/
    let select_str = format!("BEGIN; DECLARE article_cursor CURSOR FOR SELECT *, count_estimate('select * from {}') FROM  ", this.table_name);
    out.push_sql(select_str.as_str());
    if this.is_sub_query {
        out.push_sql("(");
    }
    this.query.walk_ast(out.reborrow())?;
    if this.is_sub_query {
        out.push_sql(" LIMIT ");
        out.push_bind_param::<BigInt, _>(&this.per_page)?;
        out.push_sql(" OFFSET ");
        let offset = (this.page - 1) * this.per_page;
        out.push_bind_param::<BigInt, _>(&offset)?;
        let fetch_str = format!(") t; FETCH {} FROM article_cursor; END;", this.per_page);
        out.push_sql(fetch_str.as_str());
    }
    Ok(())
}


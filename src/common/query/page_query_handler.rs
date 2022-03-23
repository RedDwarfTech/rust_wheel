use diesel::pg::Pg;
use diesel::query_builder::{AstPass, QueryFragment};
use diesel::QueryResult;
use diesel::sql_types::BigInt;
use crate::common::query::pagination::Paginated;

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

pub fn handle_big_table_query<T: QueryFragment<Pg>>(this: &Paginated<T>, mut out: AstPass<Pg>)-> QueryResult<()>{
    out.push_sql("SELECT *, count_estimate('select * from article') FROM ");
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
        out.push_sql(") t");
    }
    Ok(())
}

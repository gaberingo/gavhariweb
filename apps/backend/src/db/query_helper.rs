use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use diesel::query_dsl::methods::LimitDsl;
use diesel::query_dsl::LoadQuery;
use diesel::result::QueryResult;

pub fn load_list<'a, T, Q>(conn: &mut PgConnection, query: Q, limit: i64) -> QueryResult<Vec<T>>
where
    Q: LimitDsl,
    <Q as LimitDsl>::Output: LoadQuery<'a, PgConnection, T>,
{
    query.limit(limit).load::<T>(conn)
}

pub fn new_rec<'a, T, Tab, V>(conn: &mut PgConnection, table: Tab, values: V) -> QueryResult<T>
where
    Tab: Table,
    V: Insertable<Tab>,
    InsertStatement<Tab, V::Values>: LoadQuery<'a, PgConnection, T>
{
    diesel::insert_into(table).values(values).get_result(conn)
}

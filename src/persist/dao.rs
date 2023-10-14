use super::entity::AccountRecord;
use super::DB_POOL;
use anyhow::Result;
use sqlx::{Execute, QueryBuilder, Sqlite};

pub async fn query_record(
    master_user_name: String,
    master_password: String,
    record_name_keyword: String,
) -> Result<Vec<AccountRecord>> {
    let mut sql: QueryBuilder<Sqlite> = QueryBuilder::new("select * from account_record");
    sql.push(" where user_name = ");
    sql.push_bind(master_user_name.clone());
    sql.push(" and is_removed = ");
    sql.push_bind(false);
    if record_name_keyword != "" {
        sql.push(" and name like ");
        sql.push_bind(format!("%{}%", record_name_keyword));
    }
    sql.push(" order by name");
    println!("sql: {:?}", sql.sql());

    // let gen_sql = sql.build().sql();
    // let x: Vec<AccountRecord> = sqlx::query_as(sql.build().sql())
    //     .fetch_all(DB_POOL.get().unwrap())
    //     .await?;
    let x: Vec<AccountRecord> = sql
        .build_query_as()
        .fetch_all(DB_POOL.get().unwrap())
        .await?;
    Ok(x)
}

use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::sync::OnceLock;

pub mod entity;
pub mod dao;

pub static DB_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

pub async fn init_db(db_file: &str) -> Result<()> {
    let db_file = shellexpand::tilde(db_file).to_string();
    println!("db_file:{}", db_file);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(format!("sqlite://{}", db_file).as_str())
        .await?;
    DB_POOL.get_or_init(|| pool);
    Ok(())
}

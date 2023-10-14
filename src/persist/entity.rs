use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub salt: String,
    pub pass_token: String,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub update_time: chrono::DateTime<chrono::Utc>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct AccountRecord {
    pub id: i64,
    pub user_name: String,
    pub name: String,
    pub description: String,
    pub login_name: String,
    pub salt: String,
    pub login_password_en: String,
    pub extra_message: String,
    pub is_removed: bool,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub update_time: chrono::DateTime<chrono::Utc>,
    pub remove_time: chrono::DateTime<chrono::Utc>,
}

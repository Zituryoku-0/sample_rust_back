use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct UserInfo {
    pub user_id: String,
    pub user_name: String,
}

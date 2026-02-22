use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub password: String,
}

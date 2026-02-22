use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RequestInfo {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub password: String,
}

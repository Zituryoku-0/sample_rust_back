use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseInfo {
    pub code: String,
    pub message: String,
}

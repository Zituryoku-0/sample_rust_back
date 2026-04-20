use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::Value;
use thiserror::Error;

use crate::dto::common::{Response, ResponseInfo};

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("not found")]
    NotFound { data: Value },
    #[error("internal error")]
    Internal,
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg, data) = match &self {
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m.clone(), Value::Null),
            AppError::NotFound { data } => {
                (StatusCode::NOT_FOUND, "not found".to_string(), data.clone())
            }
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal error".to_string(),
                Value::Null,
            ),
            AppError::Db(e) => {
                tracing::error!(error = %e, "sqlx error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database error".to_string(),
                    Value::Null,
                )
            }
        };
        let body = Response {
            responseinfo: ResponseInfo {
                code: status.as_u16().to_string(),
                message: msg.to_string(),
            },
            data,
        };

        (status, Json(body)).into_response()
    }
}

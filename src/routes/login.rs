use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::dto::common::{Data, Response, ResponseInfo};

#[derive(Serialize)]
struct Login {
    user_id: String,
    user_name: String,
    login_heck: bool,
    message: String,
}

async fn login() -> Json<Response<Login>> {
    let login_data = Data {
        inner: Login {
            user_id: "sampleUserId1".to_string(),
            user_name: "sampleUserName1".to_string(),
            login_heck: true,
            message: "サンプルメッセージ".to_string(),
        },
    };
    Json(Response {
        responseinfo: ResponseInfo {
            code: "200".to_string(),
            message: "success".to_string(),
        },
        data: login_data.inner,
    })
}

pub fn router() -> Router {
    Router::new().route("/login", get(login))
}

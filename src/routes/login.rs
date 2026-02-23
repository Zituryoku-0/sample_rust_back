use axum::extract::State;
use axum::{routing::post, Json, Router};
use serde::Serialize;

use crate::dto::common::{LoginRequest, Response, ResponseInfo};
use crate::dto::user::userinfo;
use crate::error::AppError;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
struct Login {
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "userName")]
    user_name: String,
    #[serde(rename = "loginCheck")]
    login_check: bool,
    message: String,
}

async fn login(
    State(pool): State<PgPool>,
    Json(request_info): Json<LoginRequest>,
) -> Result<Json<Response<Login>>, Json<Response<Login>>> {
    let select_userinfo: userinfo::UserInfo = sqlx::query_as(
        "SELECT
        userId AS user_id,
        userName AS user_name
        FROM userinfo
        WHERE userId = $1
        AND userPassword = $2
        AND delete_flg = FALSE",
    )
    .bind(&request_info.user_id)
    .bind(&request_info.password)
    .fetch_one(&pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => {
            tracing::warn!("userinfo not found for given credentials");
            // 認証失敗または該当ユーザーなしの場合は、400相当のエラーを返す
            Json(Response {
                responseinfo: ResponseInfo {
                    code: "400".to_string(),
                    message: "error".to_string(),
                },
                data: Login {
                    user_id: "".to_string(),
                    user_name: "".to_string(),
                    login_check: false,
                    message: AppError::NotFound.to_string(),
                },
            })
        }
        other => {
            tracing::error!(error = %other, "failed to select userinfo");
            // その他エラーはINTERNAL ERRORとする
            Json(Response {
                responseinfo: ResponseInfo {
                    code: "500".to_string(),
                    message: "error".to_string(),
                },
                data: Login {
                    user_id: "".to_string(),
                    user_name: "".to_string(),
                    login_check: false,
                    message: AppError::NotFound.to_string(),
                },
            })
        }
    })?;

    // トリムする
    let trim_user_id = select_userinfo.user_id.trim();
    let trim_user_name = select_userinfo.user_name.trim();

    Ok(Json(Response {
        responseinfo: ResponseInfo {
            code: "200".to_string(),
            message: "success".to_string(),
        },
        data: Login {
            user_id: trim_user_id.to_string(),
            user_name: trim_user_name.to_string(),
            login_check: true,
            message: "ログインに成功しました。".to_string(),
        },
    }))
}

pub fn router() -> Router<PgPool> {
    Router::new().route("/login", post(login))
}

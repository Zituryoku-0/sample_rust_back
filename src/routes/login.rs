use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::dto::common::{Response, ResponseInfo};
use crate::dto::user::userinfo;
use crate::error::AppError;
use sqlx::PgPool;

#[derive(Serialize)]
struct Login {
    user_id: String,
    user_name: String,
    login_heck: bool,
    message: String,
}

async fn login() -> Result<Json<Response<Login>>, AppError> {
    // .env読込
    dotenvy::dotenv().ok();

    // DBプール作成
    let database_url = std::env::var("DATABASE_URL").map_err(|err| {
        tracing::error!(error = %err, "DATABASE_URL is not set");
        AppError::Internal
    })?;
    let pool = PgPool::connect(&database_url).await.map_err(|err| {
        tracing::error!(error = %err, "failed to connect to database");
        AppError::Internal
    })?;

    // SELECT
    let _one: i32 = sqlx::query_scalar("SELECT 1").fetch_one(&pool).await?;

    let select_userinfo: userinfo::UserInfo = sqlx::query_as(
        "SELECT
        userId AS user_id,
        userName AS user_name
        FROM userinfo
        WHERE userId = $1
        AND userPassword = $2
        AND delete_flg = FALSE",
    )
    .bind("sampleUserId1")
    .bind("abcdefgh")
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        tracing::error!(error = %err, "failed to select userinfo");
        err
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
            login_heck: true,
            message: "サンプルメッセージ".to_string(),
        },
    }))
}

pub fn router() -> Router {
    Router::new().route("/login", get(login))
}

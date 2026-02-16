mod app;
mod config;
mod dto;
mod error;
mod routes;

use crate::config::AppConfig;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    AppConfig::init_tracing();

    let cfg = AppConfig::from_env();

    // DB接続プールを起動時に一度だけ作成
    let pool = PgPool::connect(&cfg.database_url)
        .await
        .expect("failed to connect to database");

    let app = app::build_app(&cfg, pool);

    cfg.serve(app).await;
}

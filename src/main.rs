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
        .unwrap_or_else(|err| {
            tracing::error!(error = %err, "failed to connect to database");
            panic!("Database connection failed: {}", err)
        });

    let app = app::build_app(&cfg, pool);

    cfg.serve(app).await;
}

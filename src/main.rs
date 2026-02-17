mod app;
mod config;
mod dto;
mod error;
mod routes;

use sqlx::PgPool;

use crate::config::AppConfig;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    AppConfig::init_tracing();

    let cfg = AppConfig::from_env();

    // DBプールを起動時に一度だけ作成
    let pool = PgPool::connect(&cfg.database_url)
        .await
        .expect("failed to connect to database");

    let app = app::build_app(&cfg, pool);

    cfg.serve(app).await;
}

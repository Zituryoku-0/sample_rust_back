mod app;
mod config;
mod dto;
mod error;
mod routes;

use crate::config::AppConfig;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    AppConfig::init_tracing();

    let cfg = AppConfig::from_env();
    let app = app::build_app(&cfg);

    cfg.serve(app).await;
}

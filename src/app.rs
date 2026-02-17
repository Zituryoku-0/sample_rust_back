use axum::{
    http::{header, HeaderValue, Method},
    Router,
};
use sqlx::PgPool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::config::AppConfig;

pub fn build_app(cfg: &AppConfig, pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            cfg.frontend_origin
                .parse::<HeaderValue>()
                .expect("invalid FRONTEND_ORIGIN"),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    Router::new()
        .nest("/", crate::routes::router())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(pool)
}

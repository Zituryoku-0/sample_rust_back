use axum::Router;
use sqlx::PgPool;

mod health;
mod login;

pub fn router() -> Router<PgPool> {
    Router::new().merge(health::router()).merge(login::router())
}

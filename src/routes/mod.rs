use axum::Router;
use sqlx::PgPool;

mod login;

pub fn router() -> Router<PgPool> {
    Router::new().merge(login::router())
}

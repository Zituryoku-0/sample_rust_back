use axum::Router;

mod health;
mod login;

pub fn router() -> Router {
    Router::new().merge(health::router()).merge(login::router())
}

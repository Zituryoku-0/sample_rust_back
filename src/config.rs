use axum::Router;
use std::{env, net::SocketAddr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
    pub frontend_origin: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let port = env::var("PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080);
        let frontend_origin =
            env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self {
            port,
            frontend_origin,
            database_url,
        }
    }

    pub fn init_tracing() {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    pub async fn serve(self, app: Router) {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        tracing::info!("listening on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect("bind failed");
        axum::serve(listener, app).await.expect("server failed");
    }
}

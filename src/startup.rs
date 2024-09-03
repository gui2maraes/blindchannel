use crate::domain::namemap::NameMap;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;
use tokio::sync::RwLock;

use crate::routes;

pub fn application(pool: sqlx::PgPool) -> Router {
    Router::new()
        .nest("/api", routes::api::router())
        .with_state(pool)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}

pub async fn run(addr: impl ToSocketAddrs, pool: sqlx::PgPool) {
    let app = application(pool);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind to address");
    axum::serve(listener, app).await.unwrap();
}

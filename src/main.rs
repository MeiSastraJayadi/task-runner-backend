use axum::{
    Router
};
use dotenvy::dotenv;
use task_runner_backend::{db::db_connection, routes::auth_router, state::AppState};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {

    dotenv().ok();

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET not set");

    let db = db_connection().await;

    let state = AppState {
        db: Arc::new(db),
        jwt_secret
    };

    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
    

    let app = Router::new()
        .nest("/auth", auth_router::auth_router())
        .layer(
            TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new()
                    .level(Level::INFO)
                    .include_headers(false)
            )
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .include_headers(false)
            ),
        )
        .with_state(state);

    let listner = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}

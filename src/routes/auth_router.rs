use axum::{Router, routing::post};

use crate::{handlers::auth_handler, state::AppState};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
}
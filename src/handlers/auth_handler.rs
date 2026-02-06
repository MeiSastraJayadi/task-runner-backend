use axum::{Json, extract::State};

use crate::{ 
    schemas::user_auth::{AuthResponse, UserLogin, UserRegister}, 
    services::auth_services, state::AppState};


#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = UserRegister,
    responses(
        (status = 201, description = "User registered", body = AuthResponse),
        (status = 400, description = "Bad request")
    )
)]
pub async fn register(
    State(state) : State<AppState>, 
    Json(req) : Json<UserRegister>
) -> Result<Json<AuthResponse>, axum::http::StatusCode> {
    let token = auth_services::register(
        &*state.db, 
        req.email, 
        req.password, 
        req.fullname,
        &state.jwt_secret
    )
    .await 
    .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    Ok(Json(AuthResponse { token: token }))
}


#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = UserLogin,
    responses(
        (status = 201, description = "User login", body = AuthResponse),
        (status = 400, description = "Bad request")
    )
)]
pub async fn login(
    State(state) : State<AppState>, 
    Json(req) : Json<UserLogin>
) -> Result<Json<AuthResponse>, axum::http::StatusCode> {
    let token = auth_services::login(
        req.password, 
        req.email, 
        &state.db,
        &state.jwt_secret
    )
    .await
    .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    Ok(Json(AuthResponse { token: token }))
}
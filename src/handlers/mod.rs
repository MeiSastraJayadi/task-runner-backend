use crate::schemas::user_auth::{AuthResponse, UserRegister};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth_handler::register, 
        crate::handlers::auth_handler::login
    ),
    components(
        schemas(UserRegister, AuthResponse)
    ),
    tags(
        (name = "auth", description = "Authentication endpoints")
    )
)]
pub struct ApiDoc;


pub mod auth_handler;

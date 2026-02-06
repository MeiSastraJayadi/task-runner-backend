use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::db::models::users::User;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserLogin {
    pub email: String, 
    pub password: String
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserRegister {
    pub fullname: String, 
    pub email: String, 
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct UserObject {
    pub id: i64,
    pub fullname: String,
    pub email: String, 
}

#[derive(Debug, Serialize)]
pub struct Claims {
    pub id: i64, 
    pub exp: usize
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String
}

impl From<User> for UserObject {
    fn from(value: User) -> Self {
        Self { id: value.id, fullname: value.fullname.unwrap(), email: value.email.unwrap() }
    }
}
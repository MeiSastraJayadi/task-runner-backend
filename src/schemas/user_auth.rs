use serde::{Deserialize, Serialize};
use crate::db::models::users::User;

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub email: String, 
    pub password: String
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String
}

impl From<User> for UserObject {
    fn from(value: User) -> Self {
        Self { id: value.id, fullname: value.fullname.unwrap(), email: value.email.unwrap() }
    }
}
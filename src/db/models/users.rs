use sqlx::FromRow; 
use chrono::{NaiveDateTime};

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64, 
    pub email: Option<String>, 
    pub password: Option<String>, 
    pub fullname: Option<String>,
    pub created_at: Option<NaiveDateTime>
}
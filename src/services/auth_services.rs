use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};
use sqlx::PgPool;
use anyhow::{Result, anyhow};

use crate::{db::models::users::User, utils::jwt};

pub async fn register(
    db: &PgPool, 
    email: String, 
    password: String, 
    fullname: String,
    jwt_secret: &str
) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("hash password failed: {}", e))?
        .to_string(); 

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, fullname, password)
        VALUES ($1, $2, $3)
        RETURNING id, email, fullname, password, created_at
        "#,
        email,
        fullname, 
        hash
    )
    .fetch_one(db)
    .await?;
    


    Ok(jwt::generate_jwt(user.id, jwt_secret))
}

pub async fn login(
    password: String, 
    email: String, 
    db: &PgPool, 
    jwt_secret: &str
) -> Result<String> {

    let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE email = $1
            "#, 
            email
        )
        .fetch_one(db)
        .await?;

    let get_password = user.password
        .ok_or_else(|| anyhow!("created_at should not be null"))?;

    let parsed_hash = PasswordHash::new(&get_password)
        .map_err(|e| anyhow!("Failed to parse password: {}", e))?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|e| anyhow!("Failed to parse password: {}", e))?; 

    Ok(jwt::generate_jwt(user.id, jwt_secret))

}
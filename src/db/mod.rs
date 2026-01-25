pub mod models;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn db_connection() -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed connect to database")
}

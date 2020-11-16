use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;

pub async fn get_pool() -> anyhow::Result<SqlitePool, anyhow::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    println!(
        "Connected to the database at url {}",
        &env::var("DATABASE_URL")?
    );
    Ok(pool)
}
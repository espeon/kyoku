use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub async fn get_pool() -> anyhow::Result<PgPool, anyhow::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .min_connections(1)
        .max_lifetime(std::time::Duration::from_secs(10))
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    println!(
        "Connected to the database at url {}",
        &env::var("DATABASE_URL")?
    );
    Ok(pool)
}
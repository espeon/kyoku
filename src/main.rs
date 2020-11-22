mod api;
mod db;
mod index;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let path = std::env::var("MOUNT")?;

    let pool = db::get_pool().await?;
    let p_cloned = pool.clone();

    // start indexing/scanning in new thread
    tokio::spawn(async move {
        index::start(path.clone(), &path, p_cloned).await;
    });

    // start up our web server
    // dunno if i want this in a separate thread or not
    rocket(pool).await?;
    Ok(())
}

async fn rocket(pool: sqlx::Pool<sqlx::Sqlite>) -> anyhow::Result<()> {
    rocket::ignite()
        .manage(pool)
        .mount("/", rocket::routes![hello])
        .mount("/system", rocket::routes![api::system::info])
        .mount("/search", rocket::routes![api::search::main])
        .launch()
        .await?;
    Ok(())
}

#[rocket::get("/")]
async fn hello() -> &'static str {
    "hello world!"
}

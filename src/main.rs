mod index;
mod db;
mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();
    let path = "test";
    
    let pool = db::get_pool().await?;

    // start indexing/scanning in new thread
    std::thread::spawn(move || {
        index::start(path.clone(), path);
    });

    // start up our web server
    // dunno if i want this in a separate thread or not
    rocket(pool).await?;
Ok(())
}


use rocket::State;
use std::sync::atomic;

struct HitCount {
    count: atomic::AtomicUsize
}

async fn rocket(_pool: sqlx::Pool<sqlx::Sqlite>) -> anyhow::Result<()> {
    rocket::ignite()
    .manage( HitCount { count: atomic::AtomicUsize::new(0) })
    .mount("/", rocket::routes![hello])
    .mount("/", rocket::routes![count])
    .mount("/system", rocket::routes![api::system::info])
    .launch()
    .await?;
    Ok(())
}

#[rocket::get("/")]
async fn hello() -> &'static str {
    "hello world!"
}

#[rocket::get("/count")]
fn count(hit_count: State<HitCount>) -> String {
    let current_count = hit_count.count.load(atomic::Ordering::Relaxed);
    format!("Number of visits: {}", current_count)
}
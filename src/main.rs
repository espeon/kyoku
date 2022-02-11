use axum::{response::Html, routing::get, AddExtensionLayer, Router};
use sqlx::{Pool, postgres::Postgres};
use std::net::SocketAddr;

mod api;
mod db;
mod index;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok(); // ok does NOT return an error if there is no env file, which is what we want here

    let path = std::env::var("MOUNT").unwrap();

    let pool = db::get_pool().await?;
    let p_cloned = pool.clone();

    // start indexing/scanning in new thread
    tokio::spawn(async move {
        index::start(path.clone(), &path, p_cloned).await;
    });

    // start up our web server
    // dunno if i want this in a separate thread or not

    serve(pool).await?;

    Ok(())
}

async fn serve(p: Pool<Postgres>) -> anyhow::Result<()> {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/serve/a-:id", get(api::serve::serve_audio))
        .route("/track/:id", get(api::song::get_song))
        .route("/index-q0b3.json", get(api::index::index_songs))
        .layer(AddExtensionLayer::new(p));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<code>sh.kanbaru.kyoku</code>")
}

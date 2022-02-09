use axum::{response::Html, routing::get, AddExtensionLayer, Router};
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;

mod api;
mod db;
mod index;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let path = std::env!("MOUNT");

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

async fn serve(p: Pool<Sqlite>) -> anyhow::Result<()> {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/serve/a-:id", get(api::serve::serve_audio))
        .layer(AddExtensionLayer::new(p));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

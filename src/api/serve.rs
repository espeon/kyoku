use axum::{
    body::{boxed, Body, BoxBody},
    extract::{Extension, Path},
    http::{Request, Response, StatusCode},
};
use sqlx::PgPool;
use tower_http::services::fs::ServeFile;

use tower::util::ServiceExt;

pub async fn serve_audio(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = Request::builder().uri("/").body(Body::empty()).unwrap();

    let id_parsed = id.split('.').collect::<Vec<&str>>()[0].parse::<i32>().unwrap();

    match sqlx::query!(
        r#"
        SELECT path from song
        WHERE id = $1
    "#,
        id_parsed
    )
    .fetch_one(&pool)
    .await
    {
        Ok(f) => match ServeFile::new(f.path).oneshot(res).await {
            Ok(res) => Ok(res.map(boxed)),
            Err(err) => Err((
                StatusCode::NOT_FOUND,
                format!("Something went wrong when serving a file: {}", err),
            )),
        },
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            format!("Something went wrong when finding the file: {}", err),
        )),
    }
}

pub async fn serve_art(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = Request::builder().uri("/").body(Body::empty()).unwrap();

    let id_parsed = id.split('.').collect::<Vec<&str>>()[0].parse::<i32>().unwrap();

    match sqlx::query!(
        r#"
        SELECT path from song
        WHERE id = $1
    "#,
        id_parsed
    )
    .fetch_one(&pool)
    .await
    {
        Ok(f) => match ServeFile::new(f.path).oneshot(res).await {
            Ok(res) => Ok(res.map(boxed)),
            Err(err) => Err((
                StatusCode::NOT_FOUND,
                format!("Something went wrong when serving a file: {}", err),
            )),
        },
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            format!("Something went wrong when finding the file: {}", err),
        )),
    }
}

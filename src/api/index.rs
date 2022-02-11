use axum::{extract::Extension, http::StatusCode, Json};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct IndexSong {
    id: i32,
    artist_name: Option<String>,
    song_name: String,
    album_name: Option<String>,
}

pub async fn index_songs(
    Extension(pool): Extension<PgPool>,
) -> Result<axum::Json<Vec<IndexSong>>, (StatusCode, String)> {
    match  sqlx::query_as!(IndexSong,
        r#"
    SELECT
    song.id,
    song.name as song_name,
    artist.name as artist_name,
    album.name as album_name
    FROM
    song
    LEFT JOIN album ON song.album = album.id
    LEFT JOIN artist ON song.artist = artist.id
"#,
    )
    .fetch_all(&pool)
    .await{
        Ok(e) => return Ok(Json(e)),
        Err(e) => return Err(internal_error(e)),
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

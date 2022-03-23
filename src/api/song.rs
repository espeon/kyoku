use axum::{extract::{Extension, Path}, http::StatusCode, Json};
use serde::Serialize;
use sqlx::{PgPool};

use crate::api::Track;

pub async fn get_song(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<axum::Json<Vec<Track>>, (StatusCode, String)> {
    let id_parsed = id.split('.').collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
        match  sqlx::query_as!(Track,
            r#"
        SELECT song.id, number, song.name, path, album, song.artist, liked, duration, plays, lossless, song.created_at, song.updated_at, last_play, year,
        	album.name as album_name,
            artist.name as artist_name
        FROM song
        
        LEFT JOIN album ON song.album = album.id
        LEFT JOIN artist ON song.artist = artist.id

        WHERE song.id = $1
    "#, id_parsed
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

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::api::{Track, Album, AlbumRaw, ArtistPartial};

pub async fn get_album(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<axum::Json<Album>, (StatusCode, String)> {
    let id_parsed = id.split('.').collect::<Vec<&str>>()[0]
        .parse::<i32>()
        .unwrap();

    let album = match sqlx::query_as!(AlbumRaw, r#"
        SELECT album.id, album.name, album.picture, year, album.created_at, album.updated_at, 
            artist.id as artist_id, artist.name as artist_name, artist.picture as artist_picture, artist.bio as artist_bio, 
            artist.created_at as artist_created_at, artist.updated_at as artist_updated_at
        FROM album
        LEFT JOIN artist ON album.artist = artist.id
        
        WHERE album.id = $1"#, id_parsed
    )
    .fetch_one(&pool)
    .await{
        Ok(e) => e,
        Err(e) => return Err(internal_error(e)),
    };

    match  sqlx::query_as!(Track,
            r#"
        SELECT song.id, number, song.name, path, album, song.artist, liked, duration, plays, lossless, song.created_at, song.updated_at, last_play, year,
        	album.name as album_name,
            artist.name as artist_name
        FROM song
        
        LEFT JOIN album ON song.album = album.id
        LEFT JOIN artist ON song.artist = artist.id

        WHERE song.album = $1

        ORDER BY number ASC
    "#, id_parsed
        )
        .fetch_all(&pool)
        .await{
            Ok(e) => return Ok(Json(Album{
                id: album.id, 
                name: album.name,
                picture: album.picture,
                year: album.year,
                created_at: album.created_at,
                updated_at: album.updated_at,
                artist: ArtistPartial{
                    id: album.artist_id,
                    name: album.artist_name,
                    picture: album.artist_picture,
                    bio: album.artist_bio,
                    created_at: album.artist_created_at,
                    updated_at: album.artist_updated_at
                },
                tracks: Some(e)
            })),
            Err(e) => return Err(internal_error(e)),
        }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

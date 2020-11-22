use rocket::{State, response::status::NoContent};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Results {
    tracks: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    id: i64,
    name: String,
    artist: String,
    path: String,
    plays: Option<i64>,
    duration: f64,
    liked: bool,
    last_play: Option<String>, // was serde_json::Value
    year: i64,
    number: i64,
    lossless: bool,
    created_at: String,
    updated_at: Option<String>,
    artists: Vec<Artist>,
    playlists: Vec<i64>, // was serde_json::Value
    album: Album,
    genre: Option<Genre>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    id: i64,
    name: String,
    picture: String,
    year: i64,
    created_at: String,
    updated_at: Option<String>,
    artist: Artist,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Artist {
    id: i64,
    name: String,
    picture: Option<String>,
    tags: Option<Vec<String>>,
    similar: Option<Vec<String>>,
    bio: Option<String>,
    created_at: String,
    updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    id: i64,
    name: String,
    created_at: String,
    updated_at: Option<String>,
}

#[rocket::get("/")]
pub async fn main(pool: State<'_, sqlx::sqlite::SqlitePool>) -> Result<Json<Results>, NoContent> {
    let recs = sqlx::query!(
        r#"
        select id, number, name, path, album, artist, artists, liked, duration, last_play, plays, lossless, genre, created_at, updated_at
        from `song`
        where album = 1
        "#
    )
    .fetch_all(&mut pool.acquire().await.unwrap())
    .await.unwrap();

    let mut tr: Vec<Track> = vec![];

    for track in recs.into_iter() {
        let mut ar: Vec<Artist> = vec![];

        for artist_id in track.artists.unwrap().split(",").collect::<Vec<&str>>() {
            let artist = sqlx::query!(
                r#"
                select id, name, bio, picture, created_at, updated_at, similar, tags
                from `artist`
                where id = $1;
                "#,
                artist_id
            )
            .fetch_one(&mut pool.acquire().await.unwrap())
            .await
            .unwrap();

            let artist_tags = match artist.tags{
                Some(e) => Some(e.split(",").map(|s| s.to_string()).collect::<Vec<String>>()),
                None => None,
            };

            let art = Artist {
                id: artist.id.unwrap(),
                name: artist.name,
                picture: artist.picture,
                tags: artist_tags,
                similar: match artist.similar{
                    Some(e) => Some(e.split(",").map(|s| s.to_string()).collect::<Vec<String>>()),
                    None => None,
                },
                bio: artist.bio,
                created_at: artist.created_at.to_string(),
                updated_at: match track.updated_at {
                    Some(e) => Some(e.to_string()),
                    None => None,
                },
            };
            ar.push(art);
        }

        let artisto = ar[0].to_owned();

        let album = match sqlx::query!(
            r#"
            SELECT id, name, picture, year, created_at, updated_at
            FROM `album`

            where id = $1;
            
            "#,
            track.album
        )
        .fetch_one(&mut pool.acquire().await.unwrap())
        .await
        {
            Ok(album) => Album {
                id: album.id.unwrap(),
                name: album.name,
                picture: album.picture.unwrap(),
                year: match album.year{
                    Some(e) => e,
                    None => 0
                },
                artist: artisto,
                created_at: album.created_at.to_string(),
                updated_at: match track.updated_at {
                    Some(e) => Some(e.to_string()),
                    None => None,
                },
            },
            Err(e) => {
                dbg!(e);
                return Err(NoContent)
            },
        };

        let genre = match sqlx::query!(
            r#"
            SELECT id, name, created_at, updated_at
            FROM `album`

            where id = $1;
            
            "#,
            track.genre
        )
        .fetch_one(&mut pool.acquire().await.unwrap())
        .await{
            Ok(genre) => Some(Genre {
                id: genre.id.unwrap(),
                name: genre.name,
                created_at: genre.created_at.to_string(),
                updated_at: match track.updated_at {
                    Some(e) => Some(e.to_string()),
                    None => None,
                },
            }),
            Err(_) => None,
        };

        tr.push(Track {
            id: match track.id{
                Some(e) => e,
                None => 0
            },
            name: track.name,
            artist: track.artist,
            path: track.path,
            plays: track.plays,
            duration: track.duration as f64,
            liked: match track.liked{
                Some(e) => e,
                None => false
            },
            last_play: None,
            year: 2020,
            number: match track.number{
                Some(e) => e,
                None => 1
            },
            lossless: match track.lossless{
                Some(e) => e,
                None => false
            },
            created_at: track.created_at.to_string(),
            updated_at: match track.updated_at {
                Some(e) => Some(e.to_string()),
                None => None,
            },
            artists: ar,
            playlists: vec![],
            album: album,
            genre: genre,
        })
    }
    Ok(Json(Results { tracks: tr }))
}

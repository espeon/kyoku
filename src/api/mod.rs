pub mod serve;
pub mod index;
pub mod song;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    id: i32,
    name: String,
    artist: i32,
    path: String,
    plays: Option<i32>,
    duration: i32,
    liked: Option<bool>,
    last_play: Option<OffsetDateTime>, // was serde_json::Value
    year: Option<i32>,
    number: Option<i32>,
    lossless: Option<bool>,
    created_at: OffsetDateTime,
    updated_at: Option<OffsetDateTime>,
    album: i32,
    album_name: String,
    artist_name: String
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
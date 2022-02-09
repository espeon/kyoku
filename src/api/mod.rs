pub mod serve;
pub mod index;

use serde::{Deserialize, Serialize};

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
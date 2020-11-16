use std::env;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct SystemInfo {
    version: String,
    arch: String,
    node_version: String,
    num_cpus: i64,
    uptime: f64,
    free_mem: i64,
    id: i64,
    start: String,
    end: String,
    last_scan: String,
    seconds: f64,
    tracks: i64,
    albums: i64,
    artists: i64,
    size: i64,
    mount: String,
}

#[rocket::get("/info")]
pub fn info() -> Json<SystemInfo>{
    Json(SystemInfo{
        version: "0.0.1".to_string(),
        arch: "x86/64".to_string(),
        node_version: "N/A".to_string(),
        num_cpus: 64,
        uptime: 100.001,
        free_mem: 12000,
        id: 1,
        start: "N/A".to_string(),
        end: "N/A".to_string(),
        last_scan: "N/A".to_string(),
        seconds: 10.0,
        tracks: 0,
        albums: 0,
        artists: 0,
        size: 0,
        mount: env::var("MOUNT").unwrap(),
    })
}

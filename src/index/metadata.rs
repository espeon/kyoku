// most of this likely stolen from https://github.com/agersant/polaris/blob/master/src/index/metadata.rs

#[derive(Debug, PartialEq)]
pub enum AudioFormat {
    FLAC,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AudioMetadata {
    pub name: String,
    pub number: u32,
    pub duration: u32,
    pub album: String,
    pub album_artist: String,
    pub artists: Vec<String>,
    pub picture: Option<Picture>,
    pub path: std::path::PathBuf,
    pub year: Option<i32>,
    pub lossless: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Picture {
    pub bytes: Vec<u8>,
}



pub fn get_filetype(path: &std::path::PathBuf) -> Option<AudioFormat> {
    // get extension
    let extension = match path.extension() {
        Some(e) => e,
        _ => return None,
    };
    // format to string so we can match easily
    let extension = match extension.to_str() {
        Some(e) => e,
        _ => return None,
    };
    //match extension string to string options
    match extension.to_lowercase().as_str() {
        "flac" => Some(AudioFormat::FLAC),
        _ => None,
    }
}

pub async fn scan_file(path: &std::path::PathBuf, pool: sqlx::Pool<sqlx::Postgres>) {
    let data = match get_filetype(path) {
        Some(AudioFormat::FLAC) => Some(scan_flac(path, pool).await),
        None => return,
    };
    match data {
        r => println!("{}", r.unwrap()),
        //Some(Err(e)) => {
        //	println!("Error while scanning file metadata at path '{:?}': {}", path, e);
        //},
        //None => println!("none"),
    };
}

pub async fn scan_flac(path: &std::path::PathBuf, pool: sqlx::Pool<sqlx::Postgres>) -> String {
    // read da tag
    let tag = metaflac::Tag::read_from_path(path).unwrap();
    let vorbis = tag.vorbis_comments().ok_or(0).unwrap();
    // calculate the number of secs
    let mut streaminfo = tag.get_blocks(metaflac::BlockType::StreamInfo);
    let duration = match streaminfo.next() {
        Some(metaflac::Block::StreamInfo(s)) => {
            Some(s.total_samples as u32 / s.sample_rate)
        }
        _ => None,
    }
    .unwrap();
    let year = vorbis.get("DATE").and_then(|d| d[0].parse::<i32>().ok());

    let picture = tag
        .pictures().find(|&pic| matches!(pic.picture_type, metaflac::block::PictureType::CoverFront)).map(|pic| Picture {
                bytes: pic.data.to_owned(),
            });

    let metadata = AudioMetadata {
        name: vorbis.title().map(|v| v[0].clone()).unwrap(),
        number: vorbis.track().unwrap(),
        duration,
        album: vorbis.album().map(|v| v[0].clone()).unwrap(),
        album_artist: match vorbis.album_artist().map(|v| v[0].clone()) {
            Some(e) => e,
            None => vorbis.artist().map(|v| v[0].clone()).unwrap(),
        },
        artists: vorbis.artist().unwrap().to_owned(),
        picture,
        path: path.to_owned(),
        year,
        lossless: true,
    };

    crate::index::db::add_song(metadata, pool).await;

    // make it pretty~!
    let secs = time::Duration::seconds(duration as i64);
    // like this: min:sec
    let mut formatted_duration = format!(
        "{}:{:02}",
        &secs.whole_minutes(),
        &secs.whole_seconds() - (secs.whole_minutes() * 60)
    );
    // hours maybe? nah lol
    if secs.whole_hours() > 0 {
        formatted_duration = format!(
            "{}:{:02}:{:02}",
            secs.whole_hours(),
            &secs.whole_minutes(),
            &secs.whole_seconds() - (secs.whole_minutes() * 60)
        )
    }
    // TODO remove this and replace with a struct containing this and more stuff
    // or keep this and commit to db somewhere else?
    format!(
        "{}. {} by {} ({})",
        vorbis.track().unwrap_or(1),
        vorbis.title().map(|v| v[0].clone()).unwrap(),
        match vorbis.album_artist().map(|v| v[0].clone()) {
            Some(e) => e,
            None => vorbis.artist().map(|v| v[0].clone()).unwrap(),
        },
        formatted_duration
    )
}

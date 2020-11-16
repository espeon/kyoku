// most of this probably likely stolen from https://github.com/agersant/polaris/blob/master/src/index/metadata.rs

#[derive(Debug, PartialEq)]
pub enum AudioFormat {
    FLAC,
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

pub fn scan_file(path: &std::path::PathBuf) {
    let data = match get_filetype(path) {
        Some(AudioFormat::FLAC) => Some(scan_flac(path)),
        None => None,
    };
    match data {
        r => println!("{}", r.unwrap()),
        //Some(Err(e)) => {
        //	println!("Error while scanning file metadata at path '{:?}': {}", path, e);
        //},
        //None => None
    };
}

pub fn scan_flac(path: &std::path::PathBuf) -> String {
    // read da tag
    let tag = metaflac::Tag::read_from_path(path).unwrap();
    let vorbis = tag.vorbis_comments().ok_or(0).unwrap();
    // calculate the number of secs
    let mut streaminfo = tag.get_blocks(metaflac::BlockType::StreamInfo);
    let duration = match streaminfo.next() {
        Some(&metaflac::Block::StreamInfo(ref s)) => {
            Some((s.total_samples as u32 / s.sample_rate) as u32)
        }
        _ => None,
    }
    .unwrap();
    // format accordingly
    let secs = chrono::Duration::seconds(duration as i64);
    // like this: min:sec
    let mut formatted_duration = format!(
        "{}:{:02}",
        &secs.num_minutes(),
        &secs.num_seconds() - (secs.num_minutes() * 60)
    );
    // hours maybe? nah lol
    if secs.num_hours() > 0 {
        formatted_duration = format!(
            "{}:{:02}:{:02}",
            secs.num_hours(),
            &secs.num_minutes(),
            &secs.num_seconds() - (secs.num_minutes() * 60)
        )
    }
    // TODO remove this and replace with a struct containing this and more stuff
    format!(
        "{}. {} by {} ({})",
        match vorbis.track() {
            Some(e) => e,
            None => 1,
        },
        vorbis.title().map(|v| v[0].clone()).unwrap(),
        match vorbis.album_artist().map(|v| v[0].clone()) {
            Some(e) => e,
            None => vorbis.artist().map(|v| v[0].clone()).unwrap(),
        },
        formatted_duration
    )
}

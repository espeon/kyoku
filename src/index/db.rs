use crate::index::metadata::AudioMetadata;
use md5::{Digest, Md5};
use sqlx::Sqlite;

pub async fn add_song(metadata: AudioMetadata, pool: sqlx::Pool<Sqlite>) {
    // find or create new artist/s
    let artist = artist_foc(metadata.clone(), pool.clone()).await.unwrap();
    // find/create new album
    let album = album_foc(metadata.clone(), artist.clone(), pool.clone())
        .await
        .unwrap();
    // check for genre data, and if so, find/create.
    // finally, add our track
    let _song = song_foc(metadata, artist, album, pool).await.unwrap();
}

async fn artist_foc(metadata: AudioMetadata, pool: sqlx::Pool<Sqlite>) -> anyhow::Result<Vec<i32>> {
    let mut artivec: Vec<i32> = vec![];
    for arti in metadata.artists {
        dbg!(&arti);
        let _artist = match sqlx::query!(
            r#"
        select id
        from `artist`
        where name = $1;
        "#,
            arti
        )
        .fetch_all(&mut pool.acquire().await?)
        .await
        {
            Ok(e) => {
                if e.len() > 0 {
                    artivec.push(e[0].id.unwrap() as i32)
                } else {
                    let fm_info = crate::index::fm::get_artist_info(&metadata.album_artist).await?;
                    let artist_image = crate::index::spotify::get_artist_image(&arti).await?;
                    let now = chrono::offset::Utc::now().to_rfc3339();
                    let similar = fm_info.similar.join(",");
                    let tags = fm_info.tags.join(",");
                    match sqlx::query!(
                        r#"
                INSERT INTO `artist` (name, bio, picture, created_at, similar, tags)
                VALUES($1, $2, $3, $4, $5, $6);
                select id
                from `artist`
                where name = $1;
                "#,
                        arti,
                        fm_info.bio,
                        artist_image,
                        now,
                        similar,
                        tags,
                        arti
                    )
                    .fetch_all(&mut pool.acquire().await?)
                    .await
                    {
                        Ok(e) => artivec.push(e[0].id.unwrap() as i32),
                        Err(e) => return Err(anyhow::format_err!(e)),
                    };
                }
            }
            Err(e) => return Err(anyhow::format_err!(e)),
        };
    }
    Ok(artivec)
}

async fn album_foc(
    metadata: AudioMetadata,
    artist: Vec<i32>,
    pool: sqlx::Pool<Sqlite>,
) -> anyhow::Result<i32> {
    let _ = match sqlx::query!(
        r#"
        select id
        from `album`
        where name = $1;
        "#,
        metadata.album,
    )
    .fetch_all(&mut pool.acquire().await?)
    .await
    {
        Ok(e) => {
            if e.len() > 0 {
                return Ok(e[0].id.unwrap() as i32);
            } else {
                let image_url = match metadata.picture {
                    Some(e) => match save_image_md5(e.bytes).await {
                        Ok(e) => e,
                        Err(_) => "https://http.cat/450".to_string(),
                    },
                    None => "https://http.cat/404".to_string(),
                };
                let now = chrono::offset::Utc::now().to_rfc3339();
                let _ = match sqlx::query!(
                    r#"
            INSERT INTO `album` (name, artist, picture, year, created_at)
            VALUES ($1, $2, $3, $4, $5);
            select id
            from `album`
            where name = $6;
            "#,
                    metadata.album,
                    artist[0],
                    image_url,
                    metadata.year,
                    now,
                    metadata.album
                )
                .fetch_all(&mut pool.acquire().await?)
                .await
                {
                    Ok(e) => return Ok(e[0].id.unwrap() as i32),
                    Err(e) => {
                        return Err(anyhow::format_err!(e));
                    }
                };
            }
        }
        Err(e) => {
            return Err(anyhow::format_err!(e));
        }
    };
}

async fn song_foc(
    metadata: AudioMetadata,
    artist: Vec<i32>,
    album: i32,
    pool: sqlx::Pool<Sqlite>,
) -> anyhow::Result<i32> {
    let _ = match sqlx::query!(
        r#"
        select id
        from `song`
        where name = $1;
        "#,
        metadata.name,
    )
    .fetch_all(&mut pool.acquire().await?)
    .await
    {
        Ok(e) => {
            if e.len() > 0 {
                return Ok(e[0].id.unwrap() as i32);
            } else {
                let now = chrono::offset::Utc::now().to_rfc3339();
                let v = artist.into_iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");
                let p = metadata.path.to_str().unwrap();
                let n = metadata.number as i64;
                let d = metadata.duration as i64;
                let a = album as i64;
                let _ = match sqlx::query!(
                    r#"
                    INSERT INTO `song` (number, name, path, album, artist, artists, liked, duration, plays, lossless, genre, created_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 10, $11, $12);
                    select id
                    from `song`
                    where name = $2;
                    "#,
                    n,
                    metadata.name,
                    p,
                    a,
                    metadata.album_artist,
                    v,
                    false,
                    d,
                    0,
                    metadata.lossless,
                    "",
                    now,
                )
                .fetch_all(&mut pool.acquire().await?)
                .await
                {
                    Ok(e) => return Ok(e[0].id.unwrap() as i32),
                    Err(e) => {
                        return Err(anyhow::format_err!(e));
                    }
                };
            }
        }
        Err(e) => {
            return Err(anyhow::format_err!(e));
        }
    };
}

async fn save_image_md5(bytes: Vec<u8>) -> anyhow::Result<String> {
    // create a Md5 hasher instance
    let hash = Md5::digest(&bytes);

    let dest = format! {"./art/{:x}.png",&hash};
    let mut out = tokio::fs::File::create(&dest).await?;
    tokio::io::copy(&mut &*bytes, &mut out).await?;
    Ok(dest)
}

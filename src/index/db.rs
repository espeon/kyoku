use crate::index::metadata::AudioMetadata;
use md5::{Digest, Md5};
use sqlx::postgres::{Postgres};


pub async fn add_song(metadata: AudioMetadata, pool: sqlx::Pool<Postgres>) {
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

async fn artist_foc(metadata: AudioMetadata, pool: sqlx::Pool<Postgres>) -> anyhow::Result<Vec<i32>> {
    // temporary artist storage (to return)
    let mut artivec: Vec<i32> = vec![];
    for arti in metadata.artists {
        let _artist = match sqlx::query!(
            r#"
        select id
        from artist
        where name = $1;
        "#,
            arti
        )
        .fetch_all(&mut pool.acquire().await?)
        .await
        {
            Ok(e) => {
                if e.len() > 0 {
                    artivec.push(e[0].id as i32)
                } else {
                    // format and coerce

                    let fm_info = crate::index::fm::get_artist_info(&metadata.album_artist).await?;
                    let artist_image = crate::index::spotify::get_artist_image(&arti).await?;
                    //let similar = fm_info.similar.join(",");
                    let tags = fm_info.tags.join(",");
                    // insert into db
                    match sqlx::query!(
                        r#"
                INSERT INTO artist (name, bio, picture, created_at, tags)
                VALUES($1, $2, $3, $4, $5)
                RETURNING id
                "#,
                        arti,
                        fm_info.bio,
                        artist_image,
                        &time::OffsetDateTime::now_utc(),
                        tags,
                    )
                    .fetch_all(&mut pool.acquire().await?)
                    .await
                    {
                        Ok(e) => artivec.push(e[0].id as i32),
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
    pool: sqlx::Pool<Postgres>,
) -> anyhow::Result<i32> {
    // check if album already exists
    let _ = match sqlx::query!(
        r#"
        select id
        from album
        where name = $1;
        "#,
        metadata.album,
    )
    .fetch_all(&mut pool.acquire().await?)
    .await
    {
        Ok(e) => {
            // if no results, return
            if e.len() > 0 {
                return Ok(e[0].id as i32);
            } else {
                // else we insert the allbum
                // save image as <md5> OR a 404 image
                let image_url = match metadata.picture {
                    Some(e) => match save_image_md5(e.bytes).await {
                        Ok(e) => e,
                        Err(_) => "https://http.cat/450".to_string(),
                    },
                    None => "https://http.cat/404".to_string(),
                };

                // insert into database
                let _ = match sqlx::query!(
                    r#"
            INSERT INTO album (name, artist, picture, year, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id;
            "#,
                    metadata.album,
                    artist[0],
                    image_url,
                    metadata.year,
                    time::OffsetDateTime::now_utc()
                )
                .fetch_all(&mut pool.acquire().await?)
                .await
                {
                    Ok(e) => return Ok(e[0].id as i32),
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
    _artist: Vec<i32>,
    album: i32,
    pool: sqlx::Pool<Postgres>,
) -> anyhow::Result<i32> {
    // check if song exists
    let _ = match sqlx::query!(
        r#"
        select id
        from song
        where name = $1;
        "#,
        metadata.name,
    )
    .fetch_all(&mut pool.acquire().await?)
    .await
    {
        Ok(e) => {
            if e.len() > 0 {
                return Ok(e[0].id as i32);
            } else {
                // put in database

                let artist = match sqlx::query!(
                    r#"
                    SELECT id FROM artist
                    WHERE name = $1
                    "#,
                    metadata.album_artist
                )
                .fetch_one(&mut pool.acquire().await?)
                .await
                {
                    Ok(e) => e.id,
                    Err(e) => {
                        return Err(anyhow::format_err!(e));
                    }
                };

                //let v = artist
                //    .into_iter()
                //    .map(|n| n.to_string())
                //    .collect::<Vec<String>>()
                //    .join(",");
                let p = metadata.path.to_str().unwrap();
                let _ = match sqlx::query!(
                    r#"
                    INSERT INTO song (number, name, path, album, artist, liked, duration, plays, lossless, genre, created_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                    RETURNING id;
                    "#,
                    metadata.number as i32,
                    metadata.name,
                    p,
                    album as i32,
                    artist,
                    false,
                    metadata.duration as i32,
                    0 as i32,
                    metadata.lossless,
                    1,
                    time::OffsetDateTime::now_utc(),
                )
                .fetch_all(&mut pool.acquire().await?)
                .await
                {
                    Ok(e) => return Ok(e[0].id as i32),
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
    Ok(format! {"{:x}.png",&hash})
}

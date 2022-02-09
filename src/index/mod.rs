mod metadata;
mod db;
mod fm;
mod spotify;

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};

use sqlx::Sqlite;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::EventKind;
use std::{path::Path, thread, time};
use jwalk::{WalkDir};

pub async fn start<P: AsRef<Path>>(path: P, pathstr:&str, pool:sqlx::Pool<Sqlite>) {
    if Path::new(&pathstr).is_file(){
        return println!("critical error !!!!!\nthe path {} is not a folder.",&pathstr)
    }
    println!("scanning folder {:?}", &pathstr);
    scan(&path, pool.clone()).await;
    println!("watching folder {:?}", pathstr);
    if let Err(e) = watch(path, pool).await {
        println!("error: {:?}", e)
    }
}

pub async fn scan<P: AsRef<Path>>(path: P, pool:sqlx::Pool<Sqlite>) {
    // wait for other stuff to finish logging
    // will remove this sooner or later
    thread::sleep(time::Duration::from_millis(250));
    for entry in WalkDir::new(path).sort(true) {
        let ent = &entry.unwrap();
        if ent.path().is_file() {
            metadata::scan_file(&ent.path(), pool.clone()).await;
        }
      }
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    })?;

    Ok((watcher, rx))
}

pub async fn watch<P: AsRef<Path>>(path: P, pool:sqlx::Pool<Sqlite>) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => parse_event(event, pool.clone()).await,
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

async fn parse_event(event: notify::event::Event,pool:sqlx::Pool<Sqlite>) {
    match event.kind {
                                // we sleep here until windows stops messing around with our file smh!
        EventKind::Create(_) => {thread::sleep(time::Duration::from_millis(75));
                                    metadata::scan_file(&event.paths[0], pool).await
                                },
        EventKind::Remove(_) => println!("removed {}", event.paths[0].to_str().unwrap()),
        EventKind::Modify(_) =>return,
        EventKind::Access(_) =>return,
        EventKind::Any => return,
        EventKind::Other => return,
    }
}
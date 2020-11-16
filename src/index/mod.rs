mod metadata;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::EventKind;
use std::{path::Path, thread, time};
use jwalk::{WalkDir};

pub fn start<P: AsRef<Path>>(path: P, pathstr:&str) {
    if Path::new(&pathstr).is_file(){
        return println!("critical error !!!!!\nthe path {} is not a folder.",&pathstr)
    }
    println!("scanning folder {:?}", &pathstr);
    scan(&path);
    println!("watching folder {:?}", pathstr);
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

pub fn scan<P: AsRef<Path>>(path: P) {
    // wait for other stuff to finish logging
    // will remove this sooner or later
    thread::sleep(time::Duration::from_millis(250));
    for entry in WalkDir::new(path).sort(true) {
        let ent = &entry.unwrap();
        if ent.path().is_file() {
            metadata::scan_file(&ent.path());
        }
      }
}

pub fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| tx.send(res).unwrap())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => parse_event(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn parse_event(event: notify::event::Event) {
    match event.kind {
        EventKind::Create(_) => {thread::sleep(time::Duration::from_millis(75));metadata::scan_file(&event.paths[0])},
        EventKind::Remove(_) => println!("removed {}", event.paths[0].to_str().unwrap()),
        EventKind::Modify(_) =>return,
        EventKind::Access(_) =>return,
        EventKind::Any => return,
        EventKind::Other => return,
    }
}
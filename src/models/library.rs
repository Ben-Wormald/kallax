use std::{fs::read_dir, path::PathBuf, sync::Arc};

use crate::Track;

pub struct Library {
    pub tracks: Arc<Vec<Arc<Track>>>,
}

impl Library {
    pub fn new() -> Library {
        let mut tracks = vec![];
        read_tracks(PathBuf::from("/Users/ben/Music"), &mut tracks);
        let tracks = Arc::new(tracks);

        Library { tracks }
    }
}

fn read_tracks(dir: PathBuf, tracks: &mut Vec<Arc<Track>>) {
    for entry in read_dir(dir).unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            read_tracks(path, tracks);
        } else if path.extension().is_some_and(|extension| extension == "mp3") {
            tracks.push(Arc::new(Track::read(path)))
        }
    }
}

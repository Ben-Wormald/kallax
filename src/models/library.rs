use gpui::ModelContext;
use std::{fs::read_dir, path::PathBuf, sync::Arc};

use crate::{store, Track};

pub struct Library {
    pub tracks: Arc<Vec<Arc<Track>>>,
}

impl Library {
    pub fn new(_cx: &mut ModelContext<Library>) -> Library {
        let tracks = Arc::new(load_tracks());

        Library { tracks }
    }
}

fn load_tracks() -> Vec<Arc<Track>> {
    let tracks = store::load_all();

    if tracks.is_empty() {
        let mut tracks = vec![];
        read_tracks(PathBuf::from("/Users/ben/Music/_soundtracks"), &mut tracks);
        tracks
    } else {
        tracks
    }
}

fn read_tracks(dir: PathBuf, tracks: &mut Vec<Arc<Track>>) {
    for entry in read_dir(dir).unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            read_tracks(path, tracks);
        } else if path.extension().is_some_and(|extension| extension == "mp3") {
            tracks.push(Arc::new(store::Track::read_and_save(path)))
        }
    }
}

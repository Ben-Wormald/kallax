use gpui::ModelContext;
use std::{env, fs::read_dir, path::PathBuf, sync::Arc};

use crate::{store, Track};

// TODO give every track an ID and use that in events etc.?

const SUPPORTED_FILETYPES: [&str; 2] = ["mp3", "wav"];

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
        let dir = PathBuf::from(env::var("LIBRARY_DIR").unwrap_or(String::from(".")));
        let mut tracks = vec![];
        read_tracks(dir, &mut tracks);
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
        } else if path.extension().is_some_and(|extension|
            SUPPORTED_FILETYPES.contains(&extension.to_str().unwrap())
        ) {
            tracks.push(Arc::new(store::Track::read_and_save(path)))
        }
    }
}

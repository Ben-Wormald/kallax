use gpui::ModelContext;
use std::{collections::HashSet, sync::Arc};

use crate::{store, Album, Track};

// TODO give every track an ID and use that in events etc.?

pub struct Library {
    pub tracks: Arc<Vec<Arc<Track>>>,
    pub albums: Arc<Vec<Arc<Album>>>,
}

impl Library {
    pub fn new(_cx: &mut ModelContext<Library>) -> Library {
        // tracks, albums, shelves = store::load()

        let tracks = store::load();
        let albums = HashSet::new();

        // tracks.iter().for_each(|track| albums.insert(track.album_title));

        let tracks = Arc::new(tracks);
        let albums = Arc::new(albums.into_iter().collect());
        

        Library {
            tracks,
            albums,
        }
    }
}

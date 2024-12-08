use gpui::ModelContext;
use std::sync::Arc;

use crate::{store, Album, Track};

pub struct Library {
    pub tracks: Arc<Vec<Arc<Track>>>,
    pub albums: Arc<Vec<Arc<Album>>>,
}

impl Library {
    pub fn new(_cx: &mut ModelContext<Library>) -> Library {
        let (tracks, albums) = store::load();
        
        let tracks = Arc::new(tracks.into_iter().map(|track| Arc::new(track)).collect());
        let albums = Arc::new(albums.into_iter().map(|album| Arc::new(album)).collect());

        Library {
            tracks,
            albums,
        }
    }
}

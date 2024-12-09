use gpui::ModelContext;
use std::sync::Arc;

use crate::{store, Album, Artist, Track};

pub struct Library {
    pub tracks: Arc<Vec<Arc<Track>>>,
    pub albums: Arc<Vec<Arc<Album>>>,
    pub artists: Arc<Vec<Arc<Artist>>>,
}

impl Library {
    pub fn new(_cx: &mut ModelContext<Library>) -> Library {
        let (tracks, albums, artists) = store::load();
        
        let tracks = Arc::new(tracks.into_iter().map(|track| Arc::new(track)).collect());
        let albums = Arc::new(albums.into_iter().map(|album| Arc::new(album)).collect());
        let artists = Arc::new(artists.into_iter().map(|artist| Arc::new(artist)).collect());

        Library {
            tracks,
            albums,
            artists,
        }
    }
}

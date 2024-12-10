use gpui::Global;
use std::sync::Arc;

use crate::{prefix, store, Album, Artist, KallaxEntity, PlaylistShelf, SearchShelf, Track};

pub struct Library {
    pub tracks: Vec<Arc<Track>>,
    pub albums: Vec<Arc<Album>>,
    pub artists: Vec<Arc<Artist>>,
    pub searches: Vec<Arc<SearchShelf>>,
    pub playlists: Vec<Arc<PlaylistShelf>>,
}

impl Library {
    pub fn new() -> Library {
        let (tracks, albums, artists, searches, playlists) = store::load();

        // TODO make these HashMaps for performance?
        
        let tracks = tracks.into_iter().map(Arc::new).collect();
        let albums = albums.into_iter().map(Arc::new).collect();
        let artists = artists.into_iter().map(Arc::new).collect();
        let searches = searches.into_iter().map(Arc::new).collect();
        let playlists = playlists.into_iter().map(Arc::new).collect();

        Library {
            tracks,
            albums,
            artists,
            searches,
            playlists,
        }
    }

    pub fn get_track(&self, id: &str) -> Option<Arc<Track>> {
        self.tracks.iter().find(|track| track.id() == id).cloned()
    }

    pub fn get_album(&self, id: &str) -> Option<Arc<Album>> {
        self.albums.iter().find(|album| album.id() == id).cloned()
    }

    pub fn get_artist(&self, id: &str) -> Option<Arc<Artist>> {
        self.artists.iter().find(|artist| artist.id() == id).cloned()
    }

    pub fn get_search(&self, id: &str) -> Option<Arc<SearchShelf>> {
        self.searches.iter().find(|search| search.id() == id).cloned()
    }

    pub fn get_playlist(&self, id: &str) -> Option<Arc<PlaylistShelf>> {
        self.playlists.iter().find(|playlist| playlist.id() == id).cloned()
    }

    pub fn get_entity(&self, id: &str) -> Option<KallaxEntity> {
        match &id[..2] {
            prefix::TRACK => self.get_track(id).map(KallaxEntity::Track),
            prefix::SEARCH => self.get_search(id).map(KallaxEntity::Search),
            _ => panic!()
        }
    }

    pub fn execute_search(&self, search_id: &str) -> Vec<KallaxEntity> {
        if let Some(search) = self.get_search(search_id) {
            self.tracks
                .iter().map(|track| KallaxEntity::Track(Arc::clone(track)))
                .chain(self.albums.iter().map(|album| KallaxEntity::Album(Arc::clone(album))))
                .chain(self.artists.iter().map(|artist| KallaxEntity::Artist(Arc::clone(artist))))
                .chain(self.searches.iter().map(|search| KallaxEntity::Search(Arc::clone(search))))
                .chain(self.playlists.iter().map(|playlist| KallaxEntity::Playlist(Arc::clone(playlist))))
                .filter(|entity| search.matches(entity))
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Global for Library {}

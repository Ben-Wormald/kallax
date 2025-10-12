use gpui::Global;
use std::sync::Arc;

use crate::{entity_type, store, KallaxEntity};

pub struct Library {
    pub tracks: Vec<KallaxEntity>,
    pub albums: Vec<KallaxEntity>,
    pub artists: Vec<KallaxEntity>,
    pub searches: Vec<KallaxEntity>,
    pub playlists: Vec<KallaxEntity>,
}

impl Library {
    pub fn new() -> Library {
        let (tracks, albums, artists, searches, playlists) = store::load();

        // TODO make these HashMaps for performance?
        
        let tracks = tracks.into_iter().map(|e| KallaxEntity::Track(Arc::new(e))).collect();
        let albums = albums.into_iter().map(|e| KallaxEntity::Album(Arc::new(e))).collect();
        let artists = artists.into_iter().map(|e| KallaxEntity::Artist(Arc::new(e))).collect();
        let searches = searches.into_iter().map(|e| KallaxEntity::Search(Arc::new(e))).collect();
        let playlists = playlists.into_iter().map(|e| KallaxEntity::Playlist(Arc::new(e))).collect();

        Library {
            tracks,
            albums,
            artists,
            searches,
            playlists,
        }
    }

    pub fn get_track(&self, id: &str) -> Option<KallaxEntity> {
        self.tracks.iter().find(|track| track.id() == id).cloned()
    }

    pub fn get_tracks(&self, ids: &[String]) -> Vec<KallaxEntity> {
        self.tracks.iter().filter(|track| ids.contains(&track.id())).cloned().collect()
    }

    pub fn get_album(&self, id: &str) -> Option<KallaxEntity> {
        self.albums.iter().find(|album| album.id() == id).cloned()
    }

    pub fn get_artist(&self, id: &str) -> Option<KallaxEntity> {
        self.artists.iter().find(|artist| artist.id() == id).cloned()
    }

    pub fn get_search(&self, id: &str) -> Option<KallaxEntity> {
        self.searches.iter().find(|search| search.id() == id).cloned()
    }

    pub fn get_playlist(&self, id: &str) -> Option<KallaxEntity> {
        self.playlists.iter().find(|playlist| playlist.id() == id).cloned()
    }

    pub fn get_entity(&self, id: &str) -> Option<KallaxEntity> {
        match &id[..2] {
            entity_type::TRACK => self.get_track(id),
            entity_type::SEARCH => self.get_search(id),
            entity_type::ALBUM => self.get_album(id),
            entity_type::ARTIST => self.get_artist(id),
            _ => panic!()
        }
    }

    pub fn execute_search(&self, search_id: &str) -> Vec<KallaxEntity> {
        if let Some(KallaxEntity::Search(search)) = self.get_search(search_id) {
            self.tracks
                .iter()
                .chain(self.albums.iter())
                .chain(self.artists.iter())
                .chain(self.searches.iter())
                .chain(self.playlists.iter())
                .filter(|entity| search.matches(entity))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Global for Library {}

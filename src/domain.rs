use gpui::ImageData;
use std::{hash::Hash, sync::Arc};

pub struct Track {
    pub path: String,
    pub title: String,
    pub artist_name: String,
    pub album_title: String,
    pub album_artist: Option<String>,
    pub duration: Option<u32>,
    pub artwork: Option<Arc<ImageData>>,
}

pub struct Album {
    pub title: String,
    pub artist_name: String,
    pub duration: u32,
    pub artwork: Option<Arc<ImageData>>,
}
impl Hash for Album {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.artist_name.hash(state);
    }
}
impl PartialEq for Album {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.artist_name == other.artist_name
    }
}
impl Eq for Album {}

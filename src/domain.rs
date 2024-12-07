use gpui::RenderImage;
use std::{hash::Hash, sync::Arc};

pub enum Shelf {
    Search(SearchShelf),
    Playlist(PlaylistShelf),
}
impl Shelf {
    pub fn name(&self) -> &str {
        match self {
            Shelf::Search(shelf) => &shelf.name,
            Shelf::Playlist(shelf) => &shelf.name,
        }
    }
}

pub struct SearchShelf {
    pub name: String,
    pub search: (),
}

pub struct PlaylistShelf {
    pub name: String,
    pub tracks: Vec<Track>,
}

pub enum Entity {
    Track(Track),
    Album(Album),
    // Artist(Artist),
}

pub struct Track {
    pub path: String,
    pub title: String,
    pub album_id: String,
    pub artist_id: String,
    // pub artist_name: String,
    // pub album_title: String,
    // pub album_artist: Option<String>,
    pub duration: u32,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
    // pub artwork: Option<Arc<RenderImage>>,
}
impl Track {
    pub fn id(&self) -> String {
        let id = md5::compute(&self.path);
        format!("{:x}", id)
    }
}

pub struct Album {
    pub title: String,
    pub sort_title: Option<String>,
    pub album_artist: String,
    pub duration: u32,
    pub artwork: Option<Arc<RenderImage>>,
}
impl Album {
    pub fn id(&self) -> String {
        let string = format!(
            "{}{}",
            self.sort_title.as_ref().unwrap_or(&self.title),
            self.album_artist,
        );
        let id = md5::compute(string);
        format!("{:x}", id)
    }
}

impl Hash for Album {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.album_artist.hash(state);
    }
}
impl PartialEq for Album {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.album_artist == other.album_artist
    }
}
impl Eq for Album {}

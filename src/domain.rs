use gpui::RenderImage;
use std::{hash::Hash, sync::Arc};

pub trait Entity {
    fn id(&self) -> String;
}

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
impl Entity for SearchShelf {
    fn id(&self) -> String {
        let string = format!(
            "search_{}",
            self.name,
        );
        let id = md5::compute(string);
        format!("{:x}", id)
    }
}

pub struct PlaylistShelf {
    pub name: String,
    pub tracks: Vec<Track>,
}
impl Entity for PlaylistShelf {
    fn id(&self) -> String {
        let string = format!(
            "playlist_{}",
            self.name,
        );
        let id = md5::compute(string);
        format!("{:x}", id)
    }
}

// pub enum Entity {
//     Track(Track),
//     Album(Album),
//     // Artist(Artist),
// }

pub struct Track {
    pub path: String,
    pub title: String,
    pub album_id: String,
    pub artist_id: String,
    pub duration: u32,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
}
impl Entity for Track {
    fn id(&self) -> String {
        let string = format!(
            "track_{}_{}_{}",
            self.title,
            self.album_id,
            self.artist_id,
        );
        let id = md5::compute(string);
        format!("{:x}", id)
    }
}

pub struct Album {
    pub title: String,
    pub sort_title: Option<String>,
    pub artist_id: String,
    pub duration: u32,
    pub artwork: Option<Arc<RenderImage>>,
}
impl Entity for Album {
    fn id(&self) -> String {
        let string = format!(
            "album_{}_{}",
            self.sort_title.as_ref().unwrap_or(&self.title),
            self.artist_id,
        );
        let id = md5::compute(string);
        format!("{:x}", id)
    }
}
impl Hash for Album {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
impl PartialEq for Album {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
impl Eq for Album {}

pub struct Artist {
    pub name: String,
    pub sort_name: Option<String>,
}
impl Entity for Artist {
    fn id(&self) -> String {
        let string = format!(
            "artist_{}",
            self.sort_name.as_ref().unwrap_or(&self.name),
        );
        let id = md5::compute(string);
        format!("{:x}", id)
    }
}
impl Hash for Artist {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
impl PartialEq for Artist {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
impl Eq for Artist {}

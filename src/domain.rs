use gpui::ImageData;
use std::sync::Arc;

pub struct Track {
    pub path: String,
    pub title: String,
    pub artist_name: String,
    pub album_title: String,
    pub album_artist: Option<String>,
    pub duration: Option<u32>,
    pub artwork: Option<Arc<ImageData>>,
}

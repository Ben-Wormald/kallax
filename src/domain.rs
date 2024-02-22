use gpui::ImageData;
use id3::{Tag, TagLike};
use std::{path::PathBuf, sync::Arc};

use crate::*;

#[derive(Clone)]
pub struct Track {
    pub path: String,
    pub title: String,
    pub artist_name: String,
    pub album_title: String,
    pub artwork: Option<Arc<ImageData>>,
}
impl Track {
    pub fn read(path: PathBuf) -> Track {
        let path = path.to_str().unwrap().to_string();
        let tags = Tag::read_from_path(&path).unwrap();

        let title = tags.title().unwrap().to_string();
        let artist_name = tags.artist().unwrap().to_string();
        let album_title = tags.album().unwrap().to_string();
        let artwork = utils::get_image(&tags);

        Track {
            path,
            title,
            artist_name,
            album_title,
            artwork,
        }
    }
}

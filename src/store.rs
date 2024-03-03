use gpui::ImageData;
use id3::{Tag, TagLike};
use image::{jpeg::JpegDecoder, DynamicImage};
use rodio::{Decoder, Source};
use std::{fs::File, io::BufReader, path::PathBuf, sync::Arc};
use turbosql::{Turbosql, select};

use crate::domain;

#[derive(Clone, Default, Turbosql)]
pub struct Track {
    rowid: Option<i64>,
    pub path: String,
    pub title: String,
    pub artist_name: String,
    pub album_title: String,
    pub album_artist: Option<String>,
    pub duration: Option<u32>,
    pub artwork: Option<Vec<u8>>,
}
impl Track {
    pub fn read_and_save(path: PathBuf) -> domain::Track {
        let path = path.to_str().unwrap().to_string();
        let tags = Tag::read_from_path(&path).unwrap_or_default();

        let title = tags.title().unwrap_or("Unknown").to_string();
        let artist_name = tags.artist().unwrap_or("Unknown").to_string();
        let album_title = tags.album().unwrap_or("Unknown").to_string();
        let album_artist = tags.album_artist().map(str::to_string);
        let duration = read_duration(&path).or(tags.duration());
        let artwork = tags.pictures().next().map(|picture| picture.data.clone());

        let track = Track {
            rowid: None,
            path,
            title,
            artist_name,
            album_title,
            album_artist,
            duration,
            artwork,
        };

        track.insert().unwrap();

        track.to_domain()
    }

    pub fn to_domain(self) -> domain::Track {
        let artwork = self.artwork.and_then(|artwork| {
            let decoder = JpegDecoder::new(artwork.as_slice()).ok()?;
            let image = DynamicImage::from_decoder(decoder).ok()?;
            Some(Arc::new(ImageData::new(image.to_bgra8())))
        });

        domain::Track {
            path: self.path,
            title: self.title,
            artist_name: self.artist_name,
            album_title: self.album_title,
            album_artist: self.album_artist,
            duration: self.duration,
            artwork,
        }
    }
}

pub fn load_all() -> Vec<Arc<domain::Track>> {
    let tracks = select!(Vec<Track>).unwrap();
    tracks.into_iter().map(|track| Arc::new(track.to_domain())).collect()
}

fn read_duration(path: &str) -> Option<u32> {
    let file = BufReader::new(File::open(path).ok()?);
    let source = Decoder::new(file).ok()?;
    source.total_duration().map(|duration| duration.as_secs() as u32)
}

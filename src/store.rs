use gpui::RenderImage;
use id3::{Tag, TagLike};
use image::{codecs::jpeg::JpegDecoder, DynamicImage, Frame};
use rodio::{Decoder, Source};
use std::{fs::File, io::{BufReader, Cursor}, path::PathBuf, sync::Arc};
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
            let cursor: Cursor<Vec<u8>> = Cursor::new(artwork);
            let decoder = JpegDecoder::new(cursor).ok()?;
            let mut image = DynamicImage::from_decoder(decoder).ok()?.into_rgba8();
            for pixel in image.chunks_exact_mut(4) {
                let (blue, green, red, alpha) = (pixel[0], pixel[1], pixel[2], pixel[3]);
                pixel[0] = red;
                pixel[1] = green;
                pixel[2] = blue;
                pixel[3] = alpha;
            }
            Some(Arc::new(RenderImage::new(vec![Frame::new(image.into())])))
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

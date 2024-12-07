use gpui::RenderImage;
use image::{codecs::jpeg::JpegDecoder, DynamicImage, Frame};
use std::{io::Cursor, sync::Arc};
use turbosql::{select, Turbosql};

use crate::{Album, Track};

#[derive(Clone, Default, Turbosql)]
pub struct DbTrack {
    rowid: Option<i64>,
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
    // pub artwork: Option<Vec<u8>>,
}
impl DbTrack {
    fn to_domain(self) -> Track {
        // let artwork = self.artwork.and_then(|artwork| {
        //     let cursor: Cursor<Vec<u8>> = Cursor::new(artwork);
        //     let decoder = JpegDecoder::new(cursor).ok()?;
        //     let mut image = DynamicImage::from_decoder(decoder).ok()?.into_rgba8();
        //     for pixel in image.chunks_exact_mut(4) {
        //         let (blue, green, red, alpha) = (pixel[0], pixel[1], pixel[2], pixel[3]);
        //         pixel[0] = red;
        //         pixel[1] = green; // TODO don't need to swap green or alpha here?
        //         pixel[2] = blue;
        //         pixel[3] = alpha;
        //     }
        //     Some(Arc::new(RenderImage::new(vec![Frame::new(image.into())])))
        // });

        // let id = md5::compute(&self.path);
        // let id = format!("{:x}", id);

        Track {
            path: self.path,
            title: self.title,
            album_id: self.album_id,
            artist_id: self.artist_id,
            duration: self.duration,
            track_number: self.track_number,
            disc_number: self.disc_number,
        }
    }

    fn from_domain(track: &Arc<Track>) -> DbTrack {
        DbTrack {
            rowid: None,
            path: track.path.clone(),
            title: track.title.clone(),
            album_id: track.album_id.clone(),
            artist_id: track.artist_id.clone(),
            duration: track.duration,
            track_number: track.track_number,
            disc_number: track.disc_number,
        }
    }
}
impl AsRef<DbTrack> for DbTrack {
    fn as_ref(&self) -> &DbTrack {
        self
    }
}

#[derive(Clone, Default, Turbosql)]
pub struct DbAlbum {
    rowid: Option<i64>,
    pub id: String,
    pub title: String,
    pub sort_title: Option<String>,
    pub album_artist: String,
    pub artwork: Option<Vec<u8>>,
}
impl DbAlbum {
    fn to_domain(self) -> Album {
        Album {
            title: self.title,
            sort_title: self.sort_title,
            album_artist: self.album_artist,
            duration: 0,
            artwork: None,
        }
    }
}

pub fn load() -> (Vec<Arc<Track>>, Vec<Album>) {
    let tracks = select!(Vec<DbTrack>).unwrap();
    let tracks = tracks.into_iter().map(|track| Arc::new(track.to_domain())).collect();

    let albums = select!(Vec<DbAlbum>).unwrap();
    let albums = albums.into_iter().map(|album| album.to_domain()).collect();

    (tracks, albums)
}

pub fn save_batch(tracks: &Vec<Arc<Track>>) {
    let tracks: Vec<DbTrack> = tracks.iter().map(DbTrack::from_domain).collect();
    DbTrack::insert_batch(&tracks).ok();
}

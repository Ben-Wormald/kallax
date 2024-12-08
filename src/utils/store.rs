use std::collections::HashSet;

use crate::{Album, Track};
use super::{database, files::{self, TrackFile}};

pub fn load() -> (Vec<Track>, Vec<Album>) {
    let (tracks, albums) = database::load();

    // TODO sync

    if tracks.is_empty() {
        let files = files::read();
        let (tracks, albums) = from_files(files);
        database::save_tracks(&tracks);
        database::save_albums(&albums);
        (tracks, albums)
    } else {
        (tracks, albums)
    }
}

fn from_files(files: Vec<TrackFile>) -> (Vec<Track>, Vec<Album>) {
    let mut tracks = Vec::new();
    let mut albums: HashSet<Album> = HashSet::new();

    for file in files.into_iter() {
        let album = Album {
            title: file.album_title,
            sort_title: None,
            album_artist: file.album_artist.unwrap_or(file.artist_name),
            duration: file.duration,
            artwork: None,
        };

        let album_id = album.id();
        albums.insert(album);

        let track = Track {
            path: file.path,
            title: file.title,
            album_id,
            artist_id: "TODO".to_string(),
            duration: file.duration,
            track_number: file.track_number,
            disc_number: file.disc_number,
        };

        tracks.push(track);
    }

    let albums = albums.into_iter().collect();
    (tracks, albums)
}

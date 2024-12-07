use std::{collections::HashSet, sync::Arc};

use crate::{Album, Track};
use super::{database, files::{self, TrackFile}};

pub fn load() -> Vec<Arc<Track>> {
    let (tracks, albums) = database::load();

    // TODO sync

    if tracks.is_empty() {
        let files = files::read();
        let (tracks, albums) = from_files(&files);
        database::save_batch(&tracks);
        tracks
    } else {
        tracks
    }
}

fn from_files(files: &[TrackFile]) -> (Vec<Track>, Vec<Album>) {
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

        albums.insert(album);

        let track = Track {
            path: file.path,
            title: file.title,
            album_id: album.id(),
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

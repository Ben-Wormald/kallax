use std::collections::HashSet;

use crate::{domain::Entity, Album, Artist, Track};
use super::{database, files::{self, TrackFile}};

pub fn load() -> (Vec<Track>, Vec<Album>, Vec<Artist>) {
    let (tracks, albums, artists) = database::load();

    // TODO sync

    if tracks.is_empty() {
        let files = files::read();
        let (tracks, albums, artists) = from_files(files);
        database::save_tracks(&tracks);
        database::save_albums(&albums);
        database::save_artists(&artists);
        (tracks, albums, artists)
    } else {
        (tracks, albums, artists)
    }
}

fn from_files(files: Vec<TrackFile>) -> (Vec<Track>, Vec<Album>, Vec<Artist>) {
    let mut tracks = Vec::new();
    let mut albums: HashSet<Album> = HashSet::new();
    let mut artists: HashSet<Artist> = HashSet::new();

    for file in files.into_iter() {
        let artist = Artist {
            name: file.artist_name,
            sort_name: None,
        };

        let artist_id = artist.id();
        artists.insert(artist);

        let album_artist_id = if let Some(name) = file.album_artist {
            let artist = Artist {
                name,
                sort_name: None,
            };
            let album_artist_id = artist.id();
            artists.insert(artist);
            album_artist_id
        } else {
            artist_id.clone()
        };

        let album = Album {
            title: file.album_title,
            sort_title: None,
            artist_id: album_artist_id,
            duration: file.duration,
            artwork: None,
        };

        let album_id = album.id();
        albums.insert(album);

        let track = Track {
            path: file.path,
            title: file.title,
            album_id,
            artist_id,
            duration: file.duration,
            track_number: file.track_number,
            disc_number: file.disc_number,
        };

        tracks.push(track);
    }

    let albums = albums.into_iter().collect();
    let artists = artists.into_iter().collect();
    (tracks, albums, artists)
}

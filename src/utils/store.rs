use std::collections::{HashMap, HashSet};

use crate::{Album, Artist, PlaylistShelf, SearchShelf, Track};
use super::{database, files::{self, TrackFile}};

pub fn load() -> (Vec<Track>, Vec<Album>, Vec<Artist>, Vec<SearchShelf>, Vec<PlaylistShelf>) {
    let (tracks, albums, artists, searches, playlists) = database::load();

    // TODO sync

    if tracks.is_empty() {
        let files = files::read();
        let (tracks, albums, artists) = from_files(files);
        database::save_tracks(&tracks);
        database::save_albums(&albums);
        database::save_artists(&artists);
        (tracks, albums, artists, searches, playlists)
    } else {
        (tracks, albums, artists, searches, playlists)
    }
}

fn from_files(files: Vec<TrackFile>) -> (Vec<Track>, Vec<Album>, Vec<Artist>) {
    let mut tracks = Vec::new();
    let mut albums: HashMap<String, Album> = HashMap::new();
    let mut artists: HashSet<Artist> = HashSet::new();

    for file in files.into_iter() {
        let track_id = file.track_id();
        let album_id = file.album_id();
        let artist_id = file.artist_id();

        let artist = Artist {
            name: file.artist_name,
            sort_name: None,
        };

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

        albums
            .entry(album_id.clone())
            .and_modify(|album| {
                album.duration += file.duration;
                album.track_ids.push(track_id.clone());
            })
            .or_insert(Album {
                title: file.album_title,
                sort_title: None,
                artist_id: album_artist_id,
                duration: file.duration,
                artwork: None,
                track_ids: vec![track_id],
            });

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

    let albums = albums.into_values().collect();
    let artists = artists.into_iter().collect();
    (tracks, albums, artists)
}

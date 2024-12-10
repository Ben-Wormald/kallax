use id3::{Tag, TagLike};
use rodio::{Decoder, Source};
use std::{env, fs::{read_dir, File}, io::BufReader, path::PathBuf};

const SUPPORTED_FILETYPES: [&str; 2] = ["mp3", "wav"];

pub struct TrackFile {
    pub path: String,
    pub title: String,
    pub artist_name: String,
    pub album_title: String,
    pub album_artist: Option<String>,
    pub duration: u32,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
    pub artwork: Option<Vec<u8>>,
}

pub fn read() -> Vec<TrackFile> {
    let dir = PathBuf::from(env::var("LIBRARY_DIR").unwrap_or(String::from(".")));
    let mut tracks = vec![];
    read_tracks(dir, &mut tracks);
    tracks
}

fn read_tracks(dir: PathBuf, tracks: &mut Vec<TrackFile>) {
    for entry in read_dir(dir).unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            read_tracks(path, tracks);
        } else if path.extension().is_some_and(|extension|
            SUPPORTED_FILETYPES.contains(&extension.to_str().unwrap())
        ) {
            tracks.push(read_track(path))
        }
    }
}

fn read_track(path: PathBuf) -> TrackFile {
    let path = path.to_str().unwrap().to_string();
    let tags = Tag::read_from_path(&path).unwrap_or_default();

    let title = tags.title().unwrap_or("Unknown").to_string();
    let artist_name = tags.artist().unwrap_or("Unknown").to_string();
    let album_title = tags.album().unwrap_or("Unknown").to_string();
    let album_artist = tags.album_artist().map(str::to_string);
    let duration = read_duration(&path).or(tags.duration()).unwrap_or(0);
    let track_number = tags.track();
    let disc_number = tags.disc();

    let artwork = tags.pictures().next().map(|picture| picture.data.clone());

    TrackFile {
        path,
        title,
        artist_name,
        album_title,
        album_artist,
        duration,
        track_number,
        disc_number,
        artwork,
    }
}

pub fn read_duration(path: &str) -> Option<u32> {
    let file = BufReader::new(File::open(path).ok()?);
    let source = Decoder::new(file).ok()?;
    source.total_duration().map(|duration| duration.as_secs() as u32)
}

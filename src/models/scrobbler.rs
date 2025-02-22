use gpui::{Context, Entity};
use reqwest::blocking::Client;
use std::{
    env,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{Album, Artist, KallaxEntity, Library, Playback, PlaybackEvent, Track};

type Mcx<'a> = Context<'a, Scrobbler>;
type Params = Vec<(&'static str, String)>;

const API: &str = "http://ws.audioscrobbler.com/2.0/";
const USER_AGENT: &str = "kallax-music-player";

pub struct Scrobbler {
    client: Arc<Client>,
    current_track: Option<Arc<Track>>,
    time_started: Option<SystemTime>,
    time_elapsed: Duration,
}
impl Scrobbler {
    pub fn new(cx: &mut Mcx, playback: &Entity<Playback>) -> Scrobbler {
        cx.subscribe(playback, |this, _emitter, event, cx| {
            if env::var("ENABLE_SCROBBLING").is_ok_and(|var| var == "true") {
                match (**event).clone() {
                    PlaybackEvent::TrackStarted(track) => this.start(cx, &track),
                    PlaybackEvent::Paused => this.pause(),
                    PlaybackEvent::Resumed => this.resume(),
                    PlaybackEvent::TrackEnded => this.end(cx),
                }
            }
        }).detach();

        let client = Arc::new(Client::new());

        Scrobbler {
            client,
            current_track: None,
            time_started: None,
            time_elapsed: Duration::default(),
        }
    }

    fn start(&mut self, cx: &mut Mcx, track: &Arc<Track>) {
        let track = Arc::clone(track);

        self.current_track = Some(Arc::clone(&track));
        self.time_started = Some(SystemTime::now());

        self.update_now_playing(cx, &track);
    }

    fn pause(&mut self) {
        if let Some(started) = self.time_started {
            if let Ok(elapsed) = started.elapsed() {
                self.time_elapsed += elapsed;
            }
            self.time_started = None;
        }
    }

    fn resume(&mut self) {
        self.time_started = Some(SystemTime::now());
    }

    fn end(&mut self, cx: &mut Mcx) {
        if let Some(track) = self.current_track.clone() {
            if let Some(started) = self.time_started {
                if let Ok(elapsed) = started.elapsed() {
                    let four_minutes = elapsed > Duration::from_secs(4 * 60);
                    let half_track = elapsed.as_secs() > track.duration as u64;

                    if four_minutes || half_track {
                        self.scrobble(cx, &track);
                    }
                }
            }
        }

        self.current_track = None;
        self.time_started = None;
        self.time_elapsed = Duration::default();
    }

    fn update_now_playing(&mut self, cx: &mut Mcx, track: &Arc<Track>) {
        // TODO refactor these stmts duplicated in scrobble()
        let album = cx.global::<Library>().get_album(&track.album_id);
        let album = if let Some(KallaxEntity::Album(album)) = album {
            album
        } else {
            Arc::new(Album::unknown())
        };

        let artist = cx.global::<Library>().get_artist(&track.artist_id);
        let artist = if let Some(KallaxEntity::Artist(artist)) = artist {
            artist
        } else {
            Arc::new(Artist::unknown())
        };

        let album_artist = cx.global::<Library>().get_artist(&album.artist_id);
        let album_artist = if let Some(KallaxEntity::Artist(album_artist)) = album_artist {
            Some(album_artist)
        } else {
            None
        };

        let mut params = vec![
            ("method", String::from("track.updateNowPlaying")),
            ("track", track.title.clone()),
            ("artist", artist.name.clone()),
            ("album", album.title.clone()),
            ("duration", track.duration.to_string()),
        ];
        if let Some(album_artist) = album_artist {
            params.push(("albumArtist", album_artist.name.clone()));
        }

        self.send(cx, params)
            .inspect_err(|err| println!("{err}"))
            .ok();
    }

    fn scrobble(&mut self, cx: &mut Mcx, track: &Arc<Track>) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let album = cx.global::<Library>().get_album(&track.album_id);
        let album = if let Some(KallaxEntity::Album(album)) = album {
            album
        } else {
            Arc::new(Album::unknown())
        };

        let artist = cx.global::<Library>().get_artist(&track.artist_id);
        let artist = if let Some(KallaxEntity::Artist(artist)) = artist {
            artist
        } else {
            Arc::new(Artist::unknown())
        };

        let album_artist = cx.global::<Library>().get_artist(&album.artist_id);
        let album_artist = if let Some(KallaxEntity::Artist(album_artist)) = album_artist {
            Some(album_artist)
        } else {
            None
        };

        let mut params = vec![
            ("method", String::from("track.scrobble")),
            ("timestamp[0]", timestamp.to_string()),
            ("track[0]", track.title.clone()),
            ("artist[0]", artist.name.clone()),
            ("album[0]", album.title.clone()),
            ("duration[0]", track.duration.to_string()),
        ];
        if let Some(album_artist) = album_artist {
            params.push(("albumArtist[0]", album_artist.name.clone()));
        }

        self.send(cx, params)
            .inspect_err(|err| println!("{err}"))
            .ok();
    }

    fn send(&self, cx: &mut Mcx, mut params: Params) -> Result<(), env::VarError> {
        let mut auth_params = vec![
            ("api_key", env::var("LASTFM_API_KEY")?),
            ("sk", env::var("LASTFM_SESSION_KEY")?),
        ];
        params.append(&mut auth_params);

        let mut params = sign(params)?;

        params.push(("format", "json".to_string()));

        let params = params.iter()
            .map(|(k, v)| (*k, v.as_str()))
            .collect::<Vec<(&str, &str)>>();
        let body = querystring::stringify(params);

        let client = Arc::clone(&self.client);

        cx.background_executor().spawn(async move {
            client
                .post(API)
                .body(body)
                .header("User-Agent", USER_AGENT)
                .send()
                .inspect_err(|err| println!("{err}"))
                .ok();
        }).detach();

        Ok(())
    }
}

fn sign(mut params: Params) -> Result<Params, env::VarError> {
    params.sort_by(|a, b| a.0.cmp(b.0));

    let param_str = params.iter()
        .map(|(k, v)| format!("{k}{v}"))
        .collect::<Vec<String>>()
        .join("");
    let param_str = format!("{}{}", param_str, env::var("LASTFM_SECRET_KEY")?);

    let signature = md5::compute(param_str);
    let signature = format!("{:x}", signature);

    params.push(("api_sig", signature));
    Ok(params)
}

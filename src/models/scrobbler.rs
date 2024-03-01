use gpui::{Model, ModelContext};
use reqwest::blocking::Client;
use std::{env, sync::Arc};

use crate::*;

type Mcx<'a> = ModelContext<'a, Scrobbler>;

const API: &str = "http://ws.audioscrobbler.com/2.0/";

pub struct Scrobbler {
    client: Arc<Client>,
}
impl Scrobbler {
    pub fn new(playback: &Model<Playback>, cx: &mut Mcx) -> Scrobbler {
        cx.subscribe(playback, |this, _emitter, event, cx| {
            match (**event).clone() {
                PlaybackEvent::TrackStarted(track) => this.update_now_playing(cx, &track),
                PlaybackEvent::Paused => {},
                PlaybackEvent::Resumed => {},
                PlaybackEvent::TrackEnded => {},
            }
        }).detach();

        let client = Arc::new(Client::new());

        Scrobbler {
            client,
        }
    }

    fn update_now_playing(&self, cx: &mut Mcx, track: &Arc<Track>) {
        let track = Arc::clone(track);

        let mut params = vec![
            ("method", String::from("track.updateNowPlaying")),
            ("track", track.title.clone()),
            ("artist", track.artist_name.clone()),
            ("album", track.album_title.clone()),
        ];
        if let Some(album_artist) = track.album_artist.clone() {
            params.push(("albumArtist", album_artist));
        }
        if let Some(duration) = track.duration {
            params.push(("duration", duration.to_string()));
        }

        self.send(cx, params);
    }

    fn send(&self, cx: &mut Mcx, mut params: Vec<(&'static str, String)>) {
        let mut auth_params = vec![
            ("api_key", env::var("LASTFM_API_KEY").unwrap()),
            ("sk", env::var("LASTFM_SESSION_KEY").unwrap()),
        ];
        params.append(&mut auth_params);

        let mut params = sign(params);

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
                .header("User-Agent", "musicplayer")
                .send()
                .unwrap();
        }).detach();
    }
}

fn sign(mut params: Vec<(&'static str, String)>) -> Vec<(&'static str, String)> {
    params.sort_by(|a, b| a.0.cmp(&b.0));

    let param_str = params.iter()
        .map(|(k, v)| format!("{k}{v}"))
        .collect::<Vec<String>>()
        .join("");
    let param_str = format!("{}{}", param_str, env::var("LASTFM_SECRET_KEY").unwrap());

    let signature = md5::compute(param_str);
    let signature = format!("{:x}", signature);

    params.push(("api_sig", signature));
    params
}

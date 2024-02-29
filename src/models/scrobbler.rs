use gpui::{Model, ModelContext};
use reqwest::Client;
use std::{env, sync::Arc};

use crate::*;

type Mcx<'a> = ModelContext<'a, Scrobbler>;

const API: &str = "http://ws.audioscrobbler.com/2.0/";

pub struct Scrobbler {
    client: Arc<Client>,
}
impl Scrobbler {
    pub fn new(playback: &Model<Playback>, cx: &mut Mcx) -> Scrobbler {
        cx.subscribe(playback, |_subscriber, _emitter, event, _cx| {
            match (**event).clone() {
                PlaybackEvent::TrackStarted(_event) => {},
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

    fn update_now_playing(&self, cx: &mut Mcx) {
        let client = Arc::clone(&self.client);

        cx.background_executor().spawn(async move {
            let params = vec![
                ("artist", "Kelly Moran"),
                ("track", "Butterfly Phase"),
                ("album", "Butterfly Phase"),
                ("duration", "170"),
                ("api_key", &env::var("LASTFM_API_KEY").unwrap()),
                ("sk", &env::var("LASTFM_SESSION_KEY").unwrap()),
                ("method", "track.updateNowPlaying"),
                // ("format", "json"),
            ];
            client.post(API).body("hi").send().await.unwrap();
        }).detach();
    }
}

fn sign<'a>(mut params: Vec<(&'a str, &'a str)>) -> Vec<(&'a str, &'a str)> {
    params.sort_by(|a, b| a.0.cmp(b.0));

    let param_str = params.iter().map(|(k, v)| format!("{k}{v}")).collect::<Vec<String>>().join("");

    params
}

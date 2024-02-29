use gpui::{Model, ModelContext};
use reqwest::Client;
use std::env;

use crate::*;

type Mcx<'a> = ModelContext<'a, Scrobbler>;

const API: &str = "http://ws.audioscrobbler.com/2.0/";

pub struct Scrobbler {
    client: Client,
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

        let client = Client::new();

        Scrobbler {
            client,
        }
    }

    fn update_now_playing(&self, cx: &mut Mcx) {
        self.client.post(API);
    }
}

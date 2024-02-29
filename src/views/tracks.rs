use gpui::*;
use std::sync::Arc;

use crate::*;

// https://github.com/trevyn/turbosql

pub struct Tracks {
    tracks: Vec<Arc<Track>>,
}

impl Tracks {
    pub fn new() -> Tracks {
        let dir = "/Users/ben/Music/Alvvays/Antisocialites";
        // let dir = "/Users/wormab01/Music/Skee Mask - Compro";

        let tracks = std::fs::read_dir(dir).unwrap()
            .filter_map(|entry| {
                let path = entry.unwrap().path();

                if path.extension().is_some_and(|extension| extension == "mp3") {
                    Some(Arc::new(Track::read(path)))
                } else {
                    None
                }
            })
            .collect::<Vec<Arc<Track>>>();

        Tracks { tracks }
    }
}

impl Render for Tracks {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .children(
                self.tracks.clone().into_iter().map(|track|
                    elements::track(track, cx)
                )
            )
    }
}

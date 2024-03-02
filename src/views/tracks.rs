use gpui::*;
use std::sync::Arc;

use crate::*;

// https://github.com/trevyn/turbosql

pub struct Tracks {
    tracks: Arc<Vec<Arc<Track>>>,
}

impl Tracks {
    pub fn new(cx: &mut ViewContext<Tracks>, library: &Model<Library>) -> Tracks {
        cx.observe(library, |this, emitter, cx| {
            this.tracks = Arc::clone(&emitter.read(cx).tracks);
            cx.notify();
        }).detach();

        let tracks = Arc::clone(&library.read(cx).tracks);

        Tracks { tracks }
    }
}

impl Render for Tracks {
    fn render(&mut self, cx: &mut ViewContext<Tracks>) -> impl IntoElement {
        div()
            .size_full()
            .children(
                Arc::clone(&self.tracks).iter().map(|track|
                    elements::track(track, cx)
                )
            )
    }
}

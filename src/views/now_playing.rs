use gpui::*;

use crate::*;

pub struct NowPlaying {
    pub track: Option<Track>,
}

impl NowPlaying {
    pub fn new() -> NowPlaying {
        NowPlaying { track: None }
    }
}

impl Render for NowPlaying {
    fn render(&mut self, _cx: &mut ViewContext<NowPlaying>) -> impl IntoElement {
        let div = div()
            .py_1()
            .px_3()
            .border()
            .border_color(rgb(COLOUR_BORDER))
            .size_full()
            .child("Now playing:")
            .child(self.track.clone().map_or("-".to_string(), |track| track.name));

        if let Some(track) = self.track.clone() {
            if let Some(artwork) = track.artwork {
                div.child(
                    img(artwork)
                        .flex_none()
                        .w_80()
                        .h_80()
                )
            } else {
                div
            }
        } else {
            div
        }
    }
}

use gpui::*;
use std::sync::Arc;

use crate::*;

pub struct NowPlaying {
    pub tracks: Vec<Arc<Track>>,
    pub current: Option<usize>,
}

impl NowPlaying {
    pub fn new(playback: &Model<Playback>, cx: &mut ViewContext<NowPlaying>) -> NowPlaying {
        cx.observe(playback, |subscriber, emitter, cx| {
            subscriber.tracks = emitter.read(cx).queue.tracks.clone();
            subscriber.current = emitter.read(cx).queue.current;
            cx.notify();
        }).detach();

        NowPlaying {
            tracks: vec![],
            current: None,
        }
    }

    fn get_current(&self) -> Option<&Arc<Track>> {
        self.current.and_then(|current| self.tracks.get(current))
    }
}

impl Render for NowPlaying {
    fn render(&mut self, cx: &mut ViewContext<NowPlaying>) -> impl IntoElement {
        let current_track = self.get_current();

        let track_details = div()
            .id("track-details")
            // .py_1()
            // .px_3()
            .child(current_track.map_or("-".to_string(), |track| track.name.clone()));

        let track_details = if let Some(track) = current_track {
            if let Some(artwork) = track.artwork.clone() {
                track_details.child(
                    img(artwork)
                        .flex_none()
                        .w_80()
                        .h_80()
                )
            } else {
                track_details
            }
        } else {
            track_details
        };

        let track_details = track_details
            .child(div().id("pause").child("Pause").on_click(cx.listener(|_this, _event, cx| {
                cx.emit(Arc::new(UiEvent::PauseClicked))
            })))
            .child(div().id("resume").child("Resume").on_click(cx.listener(|_this, _event, cx| {
                cx.emit(Arc::new(UiEvent::ResumeClicked))
            })))
            .child(div().id("skip").child("Skip").on_click(cx.listener(|_this, _event, cx| {
                cx.emit(Arc::new(UiEvent::SkipClicked))
            })));

        div()
            .border_l()
            .border_color(rgb(theme::colours::AMSTERDAM))
            // .size_full()
            .child(
                div()
                    .flex()
                    .child(
                        div()
                            .id("now-playing")
                            .flex_1()
                            .py_1()
                            .px_3()
                            .hover(|style| style.bg(rgb(theme::colours::AMSTERDAM)))
                            .child("Now playing")
                    )
                    .child(
                        div()
                            .id("queue")
                            .flex_1()
                            .py_1()
                            .px_3()
                            .hover(|style| style.bg(rgb(theme::colours::AMSTERDAM)))
                            .child("Queue")
                    )
            )
            .child(track_details)
    }
}

use gpui::*;
use std::sync::Arc;

use crate::*;
use elements::{tab_bar, UiAction};

type Vcx<'a> = ViewContext<'a, NowPlaying>;

pub struct NowPlaying {
    pub tracks: Vec<Arc<Track>>,
    pub current: Option<usize>,
    pub selected_tab: usize,
}

impl NowPlaying {
    pub fn new(cx: &mut Vcx, playback: &Model<Playback>) -> NowPlaying {
        cx.observe(playback, |subscriber, emitter, cx| {
            subscriber.tracks = emitter.read(cx).queue.tracks.clone();
            subscriber.current = emitter.read(cx).queue.current;
            cx.notify();
        }).detach();

        NowPlaying {
            tracks: vec![],
            current: None,
            selected_tab: 0,
        }
    }

    fn get_current(&self) -> Option<&Arc<Track>> {
        self.current.and_then(|current| self.tracks.get(current))
    }

    fn render_now_playing(&mut self, cx: &mut Vcx) -> impl IntoElement {
        let current_track = self.get_current();

        let now_playing = div()
            .id("track-details")
            .flex_grow()
            .flex()
            .flex_col()
            .gap_px()
            .rounded_b_sm()
            .bg(rgb(theme::colours::AMSTERDAM))
            .child(
                div()
                    .py_1()
                    .px_3()
                    .children([
                        current_track.map_or("-".to_string(), |track| track.title.clone()),
                        current_track.map_or("-".to_string(), |track| track.artist_name.clone()),
                        current_track.map_or("-".to_string(), |track| track.album_title.clone()),
                    ])
            );

        let now_playing = if let Some(track) = current_track {
            if let Some(artwork) = track.artwork.clone() {
                now_playing.child(
                    img(artwork)
                        .flex_none()
                        .w_80()
                        .h_80()
                )
            } else {
                now_playing
            }
        } else {
            now_playing
        };

        now_playing
            .child(
                div()
                    .flex()
                    .mt_auto()
                    .gap_px()
                    .children([
                        div()
                            .id("pause")
                            .flex_1()
                            .py_1()
                            .px_3()
                            .flex()
                            .justify_center()
                            .bg(rgb(theme::colours::TOUCH))
                            .hover(|style| style.bg(rgb(theme::colours::SHALLOWS)))
                            .child("Pause")
                            .on_click(cx.listener(|_this, _event, cx|
                                cx.emit(Arc::new(UiEvent::PauseClicked))
                            )),
                        div()
                            .id("resume")
                            .flex_1()
                            .py_1()
                            .px_3()
                            .flex()
                            .justify_center()
                            .bg(rgb(theme::colours::TOUCH))
                            .hover(|style| style.bg(rgb(theme::colours::SHALLOWS)))
                            .child("Resume")
                            .on_click(cx.listener(|_this, _event, cx|
                                cx.emit(Arc::new(UiEvent::ResumeClicked))
                            )),
                        div()
                            .id("skip")
                            .flex_1()
                            .py_1()
                            .px_3()
                            .flex()
                            .justify_center()
                            .bg(rgb(theme::colours::TOUCH))
                            .hover(|style| style.bg(rgb(theme::colours::SHALLOWS)))
                            .child("Skip")
                            .on_click(cx.listener(|_this, _event, cx|
                                cx.emit(Arc::new(UiEvent::SkipClicked))
                            )),
                    ])
            )
    }

    fn render_queue(&mut self, cx: &mut Vcx) -> impl IntoElement {
        div()
            .flex_grow()
            .rounded_b_sm()
            .bg(rgb(theme::colours::AMSTERDAM))
            .children(self.tracks.iter().enumerate().map(|(index, track)| {
                div()
                    .id(ElementId::Name(track.title.clone().into()))
                    .py_1()
                    .px_3()
                    .font_weight({
                        if let Some(current) = self.current {
                            if index == current {
                                FontWeight::BOLD
                            } else {
                                FontWeight::default()
                            }
                        } else {
                            FontWeight::default()
                        }
                    })
                    .child(track.title.clone())
            }))
    }
}

impl Render for NowPlaying {
    fn render(&mut self, cx: &mut Vcx) -> impl IntoElement {
        let now_playing = div()
            .flex_grow()
            .flex()
            .flex_col()
            .max_w_80()
            .child(
                tab_bar(vec![
                    UiAction {
                        label: "Now playing",
                        event: Arc::new(UiEvent::NowPlayingTabClicked(0)),
                    },
                    UiAction {
                        label: "Queue",
                        event: Arc::new(UiEvent::NowPlayingTabClicked(1)),
                    },
                ], self.selected_tab, cx)
            );

        match self.selected_tab {
            0 => now_playing.child(self.render_now_playing(cx)),
            1 => now_playing.child(self.render_queue(cx)),
            _ => now_playing,
        }
    }
}

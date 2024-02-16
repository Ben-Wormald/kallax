use gpui::*;
use std::sync::Arc;

use crate::*;

pub struct MusicPlayer {
    tracks: View<Tracks>,
    now_playing: View<NowPlaying>,
    context_menu: View<ContextMenu>,
}

impl MusicPlayer {
    pub fn new(cx: &mut ViewContext<MusicPlayer>) -> MusicPlayer {
        let playback = cx.global::<Model<Playback>>().clone();

        let tracks = cx.new_view(|_cx| Tracks::new());
        let now_playing = cx.new_view(|_cx| NowPlaying::new());
        let context_menu = cx.new_view(|_cx| ContextMenu::new());

        cx.subscribe(&tracks, {
            let context_menu = context_menu.clone();
            move |_subscriber, _emitter, event: &Arc<UiEvent>, cx| {
                handle_event(event, cx, context_menu.clone());
            }
        }).detach();

        cx.subscribe(&context_menu, {
            move |_subscriber, emitter, event: &Arc<UiEvent>, cx| {
                handle_event(event, cx, emitter);
            }
        }).detach();

        cx.subscribe(&now_playing, {
            let context_menu = context_menu.clone();
            move |_subscriber, _emitter, event: &Arc<UiEvent>, cx| {
                handle_event(event, cx, context_menu.clone());
            }
        }).detach();

        cx.subscribe(&playback, {
            let now_playing = now_playing.clone();
            move |_subscriber, _emitter, event: &Arc<PlaybackEvent>, cx| {
                match (**event).clone() {
                    PlaybackEvent::TrackStarted(event) => {
                        now_playing.update(cx, |this, cx| {
                            this.track = Some(Arc::clone(&event.track));
                            cx.notify();
                        });
                    }
                    _ => {},
                }
            }
        }).detach();

        MusicPlayer {
            tracks,
            now_playing,
            context_menu,
        }
    }
}

impl Render for MusicPlayer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(COLOUR_BG))
            .size_full()
            .text_color(rgb(COLOUR_TEXT))
            .font("Work Sans")
            .child(self.tracks.clone())
            .child(self.now_playing.clone())
            .child(self.context_menu.clone())
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, cx| {
                this.context_menu.update(cx, |context_menu, _cx| {
                    context_menu.position = None;
                });
            }))
    }
}

fn handle_event(
    event: &Arc<UiEvent>,
    cx: &mut ViewContext<MusicPlayer>,
    context_menu: View<ContextMenu>,
) {
    match (**event).clone() {
        UiEvent::PlayClicked(event) => Playback::play(Arc::clone(&event.track), cx),
        // UiEvent::QueueClicked(event) => Playback::queue(Arc::clone(&event.track), cx),
        // UiEvent::PauseClicked => Playback::pause(cx),
        // UiEvent::ResumeClicked => Playback::resume(cx),
        // UiEvent::SkipClicked => Playback::skip(cx),
        UiEvent::RightClick(event) => {
            context_menu.update(cx, |this, cx| {
                this.items = Arc::clone(&event.items);
                this.position = Some(event.position);
                cx.notify();
            });
        },
        _ => {},
    };
}

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
        let tracks = cx.new_view(|_cx| Tracks::new());
        let now_playing = cx.new_view(|_cx| NowPlaying::new());
        let context_menu = cx.new_view(|_cx| ContextMenu::new());

        cx.subscribe(&tracks, {
            let now_playing = now_playing.clone();
            move |_subscriber, _emitter, event: &Arc<Event>, cx| {
                handle_event(event, cx, now_playing.clone());
            }
        }).detach();

        cx.subscribe(&context_menu, {
            let now_playing = now_playing.clone();
            move |_subscriber, _emitter, event: &Arc<Event>, cx| {
                handle_event(event, cx, now_playing.clone());
            }
        }).detach();

        cx.subscribe(&now_playing, {
            move |_subscriber, emitter, event: &Arc<Event>, cx| {
                handle_event(event, cx, emitter);
            }
        }).detach();

        cx.update_view(&context_menu, |_, cx| {
            cx.subscribe(&tracks, move |subscriber, _emitter, event: &RightClickEvent, _cx| {
                subscriber.position = Some(event.position);
                subscriber.is_visible = true;
                subscriber.items = Arc::clone(&event.items);
            }).detach();
        });

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
                    context_menu.is_visible = false;
                });
            }))
    }
}

fn handle_event(
    event: &Arc<Event>,
    cx: &mut ViewContext<MusicPlayer>,
    now_playing: View<NowPlaying>,
) {
    match (**event).clone() {
        Event::Play(event) => {
            Player::play(Arc::clone(&event.track), cx);
            now_playing.update(cx, |this, cx| {
                this.track = Some(Arc::clone(&event.track));
                cx.notify();
            });
        },
        Event::Queue(event) => Player::queue(Arc::clone(&event.track), cx),
        Event::Pause => Player::pause(cx),
        Event::Resume => Player::resume(cx),
        Event::Skip => Player::skip(cx),
    };
}

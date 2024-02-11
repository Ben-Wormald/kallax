use gpui::*;
use std::sync::Arc;

use crate::*;

pub struct MusicPlayer {
    _player: Model<Player>,
    tracks: View<Tracks>,
    now_playing: View<NowPlaying>,
    context_menu: View<ContextMenu>,
}

impl MusicPlayer {
    pub fn new(cx: &mut ViewContext<MusicPlayer>) -> MusicPlayer {
        let _player = cx.new_model(|_cx| Player::new());
        let tracks = cx.new_view(|_cx| Tracks::new());
        let now_playing = cx.new_view(|_cx| NowPlaying::new());
        let context_menu = cx.new_view(|_cx| ContextMenu::new());

        cx.subscribe(&tracks, move |_subscriber, _emitter, event: &PlayEvent, cx| {
            Player::play(Arc::clone(&event.track), cx);
        }).detach();

        cx.subscribe(&context_menu, move |_subscriber, _emitter, event: &Arc<Event>, cx| {
            dbg!("hi");

            let event = **event;

            match event {
                Event::PlayEvent(event) => Player::play(Arc::clone(&event.track), cx),
                _ => {}
            };
        }).detach();

        cx.update_view(&now_playing, |_, cx| {
            cx.subscribe(&tracks, move |subscriber, _emitter, event: &PlayEvent, _cx| {
                subscriber.track = Some(Arc::clone(&event.track));
            }).detach();
        });

        cx.update_view(&context_menu, |_, cx| {
            cx.subscribe(&tracks, move |subscriber, _emitter, event: &RightClickEvent, _cx| {
                subscriber.position = Some(event.position);
                subscriber.is_visible = true;
                subscriber.items = Arc::clone(&event.items);
            }).detach();
        });

        MusicPlayer {
            _player,
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

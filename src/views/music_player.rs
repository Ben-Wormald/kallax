use gpui::*;

use crate::*;

pub struct MusicPlayer {
    _player: Model<Player>,
    tracks: View<Tracks>,
    now_playing: View<NowPlaying>,
}

impl MusicPlayer {
    pub fn new(cx: &mut ViewContext<MusicPlayer>) -> MusicPlayer {
        let player = cx.new_model(|_cx| Player::new());
        let tracks = cx.new_view(|_cx| Tracks::new());
        let now_playing = cx.new_view(|_cx| NowPlaying::new());

        cx.subscribe(&tracks, move |_subscriber, _emitter, event, cx| {
            Player::play(event.track.clone(), cx);
        }).detach();

        cx.update_view(&now_playing, |_, cx| {
            cx.subscribe(&tracks, move |subscriber, _emitter, event, _cx| {
                subscriber.track = Some(event.track.clone());
            }).detach();
        });

        MusicPlayer {
            _player: player,
            tracks,
            now_playing,
        }
    }
}

impl Render for MusicPlayer {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(COLOUR_BG))
            .size_full()
            .text_color(rgb(COLOUR_TEXT))
            .font("Work Sans")
            .child(self.tracks.clone())
            .child(self.now_playing.clone())
    }
}

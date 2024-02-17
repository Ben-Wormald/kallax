use gpui::*;

mod domain;
mod elements;
mod events;
mod playback;
mod utils;
mod views;

use domain::*;
use events::*;
use playback::*;
use views::*;

const COLOUR_BG: u32 = 0x333531;
const COLOUR_BORDER: u32 = 0x1f211f;
const COLOUR_TEXT: u32 = 0xf2f4f3;

actions!(musicplayer, [Quit]);

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        let playback = cx.new_model(|cx| Playback::new(cx));
        cx.set_global(playback);

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx: &mut ViewContext<MusicPlayer>| MusicPlayer::new(cx))
        });
    });
}

use dotenv::dotenv;
use gpui::*;

mod domain;
mod elements;
mod events;
mod models;
mod theme;
mod utils;
mod views;

use domain::*;
use events::*;
use models::*;
use views::*;

actions!(musicplayer, [Quit]);

fn main() {
    dotenv().ok();

    App::new().run(|cx| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(MusicPlayer::new)
        });
    });
}

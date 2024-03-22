use dotenv::dotenv;
use gpui::*;

mod domain;
mod elements;
mod events;
mod models;
mod store;
mod theme;
mod views;

use domain::*;
use events::*;
use models::*;
use views::*;

actions!(kallax, [Quit]);

fn main() {
    dotenv().ok();

    App::new().run(|cx| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu { name: "Kallax", items: vec![MenuItem::action("Quit", Quit),]}]);

        let window_options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from("Kallax")),
                appears_transparent: true,
                ..Default::default()
            }),
            bounds: Some(Bounds {
                size: size(px(800.), px(600.)).into(),
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |cx| {
            cx.new_view(Kallax::new)
        });
    });
}

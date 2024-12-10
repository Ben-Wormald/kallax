use dotenv::dotenv;
use gpui::*;

mod domain;
mod elements;
mod events;
mod globals;
mod models;
mod utils;
mod views;

use domain::*;
use events::*;
use globals::*;
use models::*;
use utils::*;
use views::*;

actions!(kallax, [Quit]);

fn main() {
    dotenv().ok();

    App::new().run(|cx| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu { name: "Kallax".into(), items: vec![MenuItem::action("Quit", Quit),]}]);

        let window_options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from("Kallax")),
                appears_transparent: true,
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                Point { x: px(0.), y: px(0.) },
                Size { width: px(1280.), height: px(720.) },
            ))),
            focus: true,
            show: true,
            kind: WindowKind::Normal,
            is_movable: true,
            display_id: None,
            window_background: WindowBackgroundAppearance::Opaque,
            app_id: None,
            window_min_size: None,
            window_decorations: None,
        };

        cx.open_window(window_options, |cx| {
            cx.new_view(Kallax::new)
        }).ok();
    });
}

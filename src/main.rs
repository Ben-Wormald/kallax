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

actions!(kallax, [Quit, ShelfOne]);

fn main() {
    dotenv().ok();

    Application::new().run(|app| {
        app.activate(true);
        app.on_action(|_: &Quit, app| app.quit());
        app.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-1", ShelfOne, None),
        ]);
        app.set_menus(vec![
            Menu { name: "Kallax".into(), items: vec![
                MenuItem::action("Quit", Quit),
            ]},
        ]);

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
            ..Default::default()
        };

        app.open_window(window_options, |window, app| {
            app.new(|cx| Kallax::new(window, cx))
        }).ok();
    });
}

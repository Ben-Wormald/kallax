use gpui::*;
use std::sync::Arc;

use crate::{theme, KallaxEntity, Library, UiEvent};

type Vcx<'a> = Context<'a, Shelves>;

pub struct Shelves {
    shelves: Vec<KallaxEntity>,
}

impl Shelves {
    pub fn new(cx: &mut Vcx) -> Shelves {
        let searches = cx.global::<Library>().searches.iter();
        let playlists = cx.global::<Library>().playlists.iter();

        let shelves = searches.chain(playlists).cloned().collect();

        Shelves {
            shelves
        }
    }
}

impl Render for Shelves {
    fn render(&mut self, _window: &mut Window, cx: &mut Vcx) -> impl IntoElement {
        div()
            .min_h_0()
            .w_64()
            .py_0p5()
            .px_2()
            .flex()
            .flex_col()
            .child(div().flex().justify_center().child(String::from("SHELVES")))
            .child(div()
                .children(
                    self.shelves.iter().enumerate().map(|(i, shelf)| render_shelf(shelf, i + 1, cx))
                )
            )
    }
}

fn render_shelf(shelf: &KallaxEntity, index: usize, cx: &mut Vcx) -> impl IntoElement {
    let shelf_id = shelf.id();

    let shelf_element = div()
        .id(ElementId::Name(shelf_id.clone().into()))
        .py_0p5()
        .px_2()
        .flex()
        .gap_2()
        .hover(|s| s.bg(rgb(theme::colours::SMOTHER)))
        .on_click(cx.listener(move |_this, _event, _window, cx| {
            cx.emit(Arc::new(UiEvent::EntityOpened(shelf_id.clone())))
        }));

    let shelf_element = if index <= 9 {
        let hotkey = format!("⌘ {}", index);
        shelf_element
            .child(div()
                .w_10()
                .flex()
                .justify_center()
                .rounded_md()
                .border_1()
                .border_color(rgb(theme::colours::STILL))
                .child(hotkey))
    } else {
        shelf_element
    };

    shelf_element
        .child(shelf.name().to_string())
}

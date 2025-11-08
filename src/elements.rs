use gpui::*;
use std::sync::Arc;

pub mod button;
pub mod hotkey;
pub mod input;
pub mod list_entity;

pub use button::*;
pub use hotkey::*;
pub use input::*;
pub use list_entity::*;

use crate::*;

pub struct UiAction {
    pub label: &'static str,
    pub event: Arc<UiEvent>,
}

/*
pub fn track(track: &Arc<Track>, cx: &mut Context<Tracks>) -> impl IntoElement {
    let on_click = cx.listener({
        let track = Arc::clone(track);
        move |_this, _event, _window, cx| cx.emit(UiEvent::play(&track))
    });
    
    let on_right_click = cx.listener({
        let track = Arc::clone(track);
        move |_this, event: &MouseDownEvent, _window, cx: &mut Context<Tracks>| {
            cx.emit(Arc::new(UiEvent::RightClick(RightClickEvent {
                position: event.position,
                items: Arc::new(vec![
                    ContextMenuItem {
                        label: "Play",
                        event: UiEvent::play(&track),
                    },
                    ContextMenuItem {
                        label: "Queue",
                        event: UiEvent::queue(&track),
                    },
                ]),
            })));
        }
    });

    div()
        .id(ElementId::Name(track.title.clone().into()))
        .flex()
        .gap_3()
        .h_8()
        .items_center()
        .rounded(px(1.))
        .hover(|style| style.bg(rgb(theme::colours::TOUCH)))
        .child(
            div()
                .flex()
                .w_8()
                .justify_end()
                .text_color(rgb(theme::colours::YOUTH))
                .child(track.track_number.map_or(
                    String::from(""),
                    |track_number| track_number.to_string(),
                ))
        )
        .child(
            div().child(track.title.clone())
        )
        .on_click(on_click)
        .on_mouse_down(MouseButton::Right, on_right_click)
}
*/

/*
pub fn album(album: &Arc<Album>, cx: &mut Context<Albums>) -> impl IntoElement {
    let element = div()
        .id(ElementId::Name(album.id().into()))
        .size_64();

    let element = if let Some(artwork) = &album.artwork {
        element.child(
            img(Arc::clone(artwork))
                .rounded(px(1.))
                .size_full()
        )
    } else {
        element
    };

    element
        .on_click({
            let album = Arc::clone(album);
            cx.listener(move |_this, _event, _window, cx| {
                cx.emit(Arc::new(UiEvent::EntityOpened(album.id())));
            })
        })
}
*/

pub fn tab_bar<V: EventEmitter<Arc<UiEvent>>>(
    tabs: Vec<UiAction>,
    selected: usize,
    cx: &mut Context<V>,
) -> impl IntoElement {
    div()
        .flex()
        .children(tabs.into_iter().enumerate().map(|(index, item)| {
            let tab = div()
                .id(item.label)
                .flex_1()
                .py_1()
                .px_3()
                .flex()
                .justify_center()
                .text_color(rgb(theme::colours::SHALLOWS))
                .rounded_t_sm()
                .child(item.label);

            if index == selected {
                tab.bg(rgb(theme::colours::SMOTHER))
            } else {
                tab.bg(rgb(theme::colours::WINTER))
                    .hover(|style| style
                        .bg(rgb(theme::colours::YOUTH))
                    )
                    .on_click(cx.listener(move |_this, _event, _window, cx| {
                        cx.emit(Arc::clone(&item.event));
                    }))
            }
        }))
}

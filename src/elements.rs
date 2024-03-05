use gpui::*;
use std::sync::Arc;

use crate::*;

use context_menu::ContextMenuItem;

pub fn track(track: &Arc<Track>, cx: &mut ViewContext<Tracks>) -> impl IntoElement {
    let track = Arc::clone(track);

    div()
        .id(ElementId::Name(track.title.clone().into()))
        .py_1()
        .px_3()
        .rounded(px(1.))
        .hover(|style| style.bg(rgb(theme::colours::TOUCH)))
        .child(track.title.clone())
        .on_click(cx.listener({
            let track = Arc::clone(&track);
            move |_this, _event, cx| cx.emit(UiEvent::play(&track))
        }))
        .on_mouse_down(
            MouseButton::Right,
            cx.listener(move |_this, event: &MouseDownEvent, cx: &mut ViewContext<Tracks>| {
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
            })
        )
}

pub fn album(album: &Album, cx: &mut ViewContext<Albums>) -> impl IntoElement {
    let element = div()
        .id("album");

    let element = if let Some(artwork) = &album.artwork {
        element.child(
            img(Arc::clone(artwork))
                        .flex_none()
                        .w_80()
                        .h_80()
        )
    } else {
        element
    };

    element.child(album.title.clone())
}

pub struct TabBarItem {
    pub label: &'static str,
    pub event: Arc<UiEvent>,
}

pub fn tab_bar<V: EventEmitter<Arc<UiEvent>>>(
    tabs: Vec<TabBarItem>,
    selected: usize,
    cx: &mut ViewContext<V>,
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
                .child(item.label);

            if index == selected {
                tab
                    .bg(rgb(theme::colours::AMSTERDAM))
                    .text_color(rgb(theme::colours::WINTER))
                    .rounded_t_sm()
            } else {
                tab
                    .text_color(rgb(theme::colours::AMSTERDAM))
                    .rounded_t_sm()
                    .hover(|style| style
                        .rounded_sm()
                        .bg(rgb(theme::colours::YOUTH))
                        .text_color(rgb(theme::colours::SHALLOWS))
                    ).on_click(cx.listener(move |_this, _event, cx| {
                        cx.emit(Arc::clone(&item.event));
                    }))
            }
        }))
}

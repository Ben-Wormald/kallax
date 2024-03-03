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
        .hover(|style| style.bg(rgb(theme::colours::AMSTERDAM)))
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
        .gap(px(1.))
        .bg(rgb(theme::colours::SHALLOWS))
        .children(tabs.into_iter().enumerate().map(|(index, item)| {
            div()
                .id(item.label)
                .flex_1()
                .py_1()
                .px_3()
                .flex()
                .justify_center()
                .bg(rgb(
                    if index == selected {
                        theme::colours::TOUCH
                    } else {
                        theme::colours::AMSTERDAM
                    }
                ))
                .border_b_1()
                .border_color(rgb(
                    if index == selected {
                        theme::colours::TOUCH
                    } else {
                        theme::colours::SHALLOWS
                    }
                ))
                .hover(|style| style
                    .bg(rgb(theme::colours::SHALLOWS))
                    .border_color(rgb(theme::colours::SHALLOWS))
                )
                .on_click(cx.listener(move |_this, _event, cx| {
                    cx.emit(Arc::clone(&item.event));
                }))
                .child(item.label)
        }))
}

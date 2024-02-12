use gpui::*;
use std::sync::Arc;

use crate::*;

use context_menu::ContextMenuItem;

pub fn track(track: Arc<Track>, cx: &mut ViewContext<Tracks>) -> impl IntoElement {
    div()
        .id(ElementId::Name(track.name.clone().into()))
        .py_1()
        .px_3()
        .hover(|style| style.bg(rgb(COLOUR_BORDER)))
        .child(track.name.clone())
        .on_click(cx.listener({
            let track = Arc::clone(&track);
            move |_this, _event, cx| cx.emit(Event::play(&track))
        }))
        .on_mouse_down(
            MouseButton::Right,
            cx.listener(move |_this, event: &MouseDownEvent, cx: &mut ViewContext<Tracks>| {
                cx.emit(RightClickEvent {
                    position: event.position,
                    items: Arc::new(vec![
                        ContextMenuItem {
                            label: "Play".to_string(),
                            event: Event::play(&track),
                        },
                        ContextMenuItem {
                            label: "Queue".to_string(),
                            event: Event::queue(&track),
                        },
                    ]),
                });
            })
        )
}

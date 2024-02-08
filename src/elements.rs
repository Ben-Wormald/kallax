use gpui::*;
use std::sync::Arc;

use crate::*;

pub fn track(track: Arc<Track>, cx: &mut ViewContext<Tracks>) -> impl IntoElement {
    div()
        .id(ElementId::Name(track.name.clone().into()))
        .py_1()
        .px_3()
        .hover(|style| style.bg(rgb(COLOUR_BORDER)))
        .child(track.name.clone())
        .on_click(cx.listener({
            let track = track.clone();
            move |_this, _event, cx| {
                let track = track.clone();
                cx.emit(PlayEvent { track })
            }
        }))
        .on_mouse_down(MouseButton::Right, cx.listener(move |_this, event, cx| {
            //
        }))
}

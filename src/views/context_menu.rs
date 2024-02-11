use gpui::*;
use std::sync::Arc;

use crate::*;

pub struct ContextMenu {
    pub is_visible: bool,
    pub position: Option<Point<Pixels>>,
    pub items: Arc<Vec<ContextMenuItem>>,
}

impl ContextMenu {
    pub fn new() -> ContextMenu {
        ContextMenu {
            is_visible: false,
            position: None,
            items: Arc::new(vec![]),
        }
    }
}

impl Render for ContextMenu {
    fn render(&mut self, cx: &mut ViewContext<ContextMenu>) -> impl IntoElement {
        if self.is_visible {
            overlay()
                .position(self.position.unwrap())
                .child(
                    div()
                        .flex_col()
                        .bg(rgb(COLOUR_BG))
                        .children(self.items.iter().map(|item|
                            div()
                                .id(ElementId::Name(item.label.clone().into()))
                                .py_1()
                                .px_3()
                                .hover(|style| style.bg(rgb(COLOUR_BORDER)))
                                .child(item.label.clone())
                                .on_mouse_down(MouseButton::Left, cx.listener({
                                    let event = Arc::clone(&item.event);
                                    move |_this, _event, cx| {
                                        dbg!("on_click");
                                        let event = Arc::clone(&event);
                                        cx.emit(event);
                                    }
                                }))
                        ))
                )
        } else {
            overlay()
        }
    }
}

pub struct ContextMenuItem {
    pub label: String,
    pub event: Arc<Event>,
}

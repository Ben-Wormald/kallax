use gpui::*;
use std::sync::Arc;

use crate::*;

pub struct ContextMenu {
    pub position: Option<Point<Pixels>>,
    pub items: Arc<Vec<ContextMenuItem>>,
}

impl ContextMenu {
    pub fn new() -> ContextMenu {
        ContextMenu {
            position: None,
            items: Arc::new(Vec::new()),
        }
    }
}

impl Render for ContextMenu {
    fn render(&mut self, cx: &mut ViewContext<ContextMenu>) -> impl IntoElement {
        if let Some(position) = self.position {
            overlay()
                .position(position)
                .child(
                    div()
                        .flex_col()
                        .bg(rgb(theme::colours::TOUCH))
                        .border()
                        .border_color(rgb(theme::colours::AMSTERDAM))
                        .children(self.items.iter().map(|item|
                            div()
                                .id(ElementId::Name(item.label.into()))
                                .py_1()
                                .px_3()
                                .hover(|style| style.bg(rgb(theme::colours::AMSTERDAM)))
                                .child(item.label)
                                .on_mouse_down(MouseButton::Left, cx.listener({
                                    let event = Arc::clone(&item.event);
                                    move |_this, _event, cx| {
                                        cx.emit(Arc::clone(&event));
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
    pub label: &'static str,
    pub event: Arc<UiEvent>,
}

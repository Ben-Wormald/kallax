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
                        .border_1()
                        .border_color(rgb(theme::colours::SHALLOWS))
                        .bg(rgb(theme::colours::SMOTHER))
                        .p(px(2.))
                        .rounded_md()
                        .rounded_tl_none()
                        .child(
                            div()
                                .rounded_sm()
                                .rounded_tl_none()
                                .bg(rgb(theme::colours::AMSTERDAM))
                                .p(px(1.))
                                .children(self.items.iter().map(|item|
                                    div()
                                        .id(ElementId::Name(item.label.into()))
                                        .py_1()
                                        .px_3()
                                        .rounded(px(1.))
                                        .hover(|style| style.bg(rgb(theme::colours::TOUCH)))
                                        .child(item.label)
                                        .on_mouse_down(MouseButton::Left, cx.listener({
                                            let event = Arc::clone(&item.event);
                                            move |this, _event, cx| {
                                                cx.emit(Arc::clone(&event));
                                                this.position = None;
                                                cx.notify();
                                            }
                                        }))
                                ))
                        )
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

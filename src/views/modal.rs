use gpui::*;

use crate::*;

pub struct Modal {
    pub is_visible: bool,
}

impl Modal {
    pub fn new() -> Modal {
        Modal {
            is_visible: false,
        }
    }
}

impl Render for Modal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .absolute()
            .size_full()
            .top_0()
            .left_0()
            .z_index(100)
            .bg(rgba(0x03030380))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .child("hi")
            )
    }
}

use gpui::*;

use crate::*;

pub struct ContextMenu {
    pub is_visible: bool,
    pub position: Option<Point<Pixels>>,
}

impl ContextMenu {
    pub fn new() -> ContextMenu {
        ContextMenu {
            is_visible: false,
            position: None,
        }
    }
}

impl Render for ContextMenu {
    fn render(&mut self, _cx: &mut ViewContext<ContextMenu>) -> impl IntoElement {
        overlay()
    }
}

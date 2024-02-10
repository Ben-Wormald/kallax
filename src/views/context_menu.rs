use gpui::*;

pub struct ContextMenu {
    pub is_visible: bool,
    pub position: Option<Point<Pixels>>,
    pub items: Vec<String>,
}

impl ContextMenu {
    pub fn new() -> ContextMenu {
        ContextMenu {
            is_visible: false,
            position: None,
            items: vec![],
        }
    }
}

impl Render for ContextMenu {
    fn render(&mut self, _cx: &mut ViewContext<ContextMenu>) -> impl IntoElement {
        if self.is_visible {
            overlay().children(self.items.clone().into_iter()).position(self.position.unwrap())
        } else {
            overlay()
        }
    }
}

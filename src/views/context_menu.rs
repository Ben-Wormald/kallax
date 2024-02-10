use gpui::*;

pub struct ContextMenu {
    pub is_visible: bool,
    pub position: Option<Point<Pixels>>,
    pub items: Vec<ContextMenuItem>,
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
            overlay()
                .position(self.position.unwrap())
                .children(self.items.iter().map(|item| item.label.clone()))
        } else {
            overlay()
        }
    }
}

pub struct ContextMenuItem {
    pub label: String,
    pub action: Box<dyn Fn(usize) -> usize>,
}

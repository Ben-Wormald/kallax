use std::sync::Arc;

use gpui::*;

use crate::{elements::UiAction, theme};

pub struct Dropdown {
    is_open: bool,
    label: &'static str,
    options: Vec<UiAction>,
    selected: Option<usize>,
}

impl Dropdown {
    pub fn new(label: &'static str, options: Vec<UiAction>) -> Dropdown {
        // TODO random ID

        Dropdown {
            is_open: false,
            label,
            options,
            selected: None,
        }
    }
}

impl Render for Dropdown {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let label = format!(
            "{}{}",
            self.label,
            if let Some(selected) = self.selected {
                format!(": {}", self.options.get(selected).unwrap().label)
            } else {
                String::from("")
            },
        );

        let element = div()
            .child(
                div()
                    .id("dropdown")
                    .rounded(px(1.))
                    .py_1()
                    .px_3()
                    .bg(rgb(theme::colours::TOUCH))
                    .hover(|style| style.bg(rgb(theme::colours::HUMAN)))
                    .child(label)
                    .on_click(cx.listener(|this, _event, cx| {
                        this.is_open = !this.is_open;
                        cx.notify();
                    }))
            );

        if self.is_open {
            element.child(
                div()
                    .children(self.options.iter().map(|option| {
                        div()
                            .id(option.label)
                            .child(option.label)
                            .on_click({
                                let event = Arc::clone(&option.event);
                                cx.listener(move |_this, _event, cx| {
                                    cx.emit(Arc::clone(&event));
                                })
                            })
                    }))
            )
        } else {
            element
        }
    }
}

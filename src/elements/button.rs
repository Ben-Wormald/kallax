use gpui::*;

use crate::{elements, utils::theme};

pub fn button(id: &'static str, label: &'static str, hotkey: Option<String>) -> Stateful<Div> {
    let mut button = div()
        .id(id)
        .flex_1()
        .py_1()
        .px_1()
        .flex()
        .gap_2()
        .rounded_md()
        .justify_center()
        .border_1()
        .border_color(rgb(theme::colours::TOMORROW))
        .bg(rgb(theme::colours::YOUTH))
        .hover(|style| style.bg(rgb(theme::colours::SMOTHER)));

    if let Some(hotkey) = hotkey {
        button = button.child(elements::hotkey::hotkey(hotkey));
    }

    button.child(label)
}

use gpui::*;

use crate::utils::theme;

pub fn hotkey(label: String) -> impl IntoElement {
    div()
        .w_10()
        .flex()
        .justify_center()
        .rounded_md()
        .border_1()
        .border_color(rgb(theme::colours::STILL))
        .bg(rgb(theme::colours::WINTER))
        .child(label)
}
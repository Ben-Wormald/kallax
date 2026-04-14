use gpui::*;
use std::sync::Arc;

pub mod button;
pub mod hotkey;
pub mod input;
pub mod list_entity;

pub use button::*;
pub use hotkey::*;
pub use input::*;
pub use list_entity::*;

use crate::*;

pub struct UiAction {
    pub label: &'static str,
    pub event: Arc<UiEvent>,
}

pub fn tab_bar<V: EventEmitter<Arc<UiEvent>>>(
    tabs: Vec<UiAction>,
    selected: usize,
    cx: &mut Context<V>,
) -> impl IntoElement {
    div()
        .flex()
        .children(tabs.into_iter().enumerate().map(|(index, item)| {
            let tab = div()
                .id(item.label)
                .flex_1()
                .py_1()
                .px_3()
                .flex()
                .justify_center()
                .text_color(rgb(theme::colours::SHALLOWS))
                .rounded_t_sm()
                .child(item.label);

            if index == selected {
                tab.bg(rgb(theme::colours::SMOTHER))
            } else {
                tab.bg(rgb(theme::colours::WINTER))
                    .hover(|style| style
                        .bg(rgb(theme::colours::YOUTH))
                    )
                    .on_click(cx.listener(move |_this, _event, _window, cx| {
                        cx.emit(Arc::clone(&item.event));
                    }))
            }
        }))
}

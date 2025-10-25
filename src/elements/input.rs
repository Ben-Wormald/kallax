use gpui::*;

use crate::utils::theme;

pub struct Input {
    focus_handle: FocusHandle,
    value: String,
    id: &'static str,
}

impl Input {
    pub fn new(id: &'static str, cx: &mut Context<Input>) -> Input {
        Input {
            focus_handle: cx.focus_handle(),
            value: String::new(),
            id,
        }
    }
}

impl Render for Input {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id(self.id)
            .on_click(cx.listener(|this, _event, window, cx| {
                this.focus_handle.focus(window);
                cx.notify();
            }))
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                if let Some(key) = &event.keystroke.key_char {
                    this.value += key;
                    cx.notify();
                }
            }))
            .w_128()
            .h_12()
            .bg(rgb(theme::colours::SMOTHER))
            .cursor_text()
            .child(self.value.clone())
    }
}
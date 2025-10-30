use gpui::{prelude::FluentBuilder, *};

use crate::utils::theme;

pub struct Input {
    focus_handle: FocusHandle,
    value: String,
    cursor: usize,
    selection: Option<(usize, usize)>,
    id: &'static str,
    on_submit: Box<dyn Fn(&str, &mut Context<Self>)>
}

impl Input {
    pub fn new(id: &'static str, cx: &mut Context<Input>, on_submit: impl Fn(&str, &mut Context<Self>) + 'static) -> Input {
        Input {
            focus_handle: cx.focus_handle(),
            value: String::new(),
            cursor: 0,
            selection: None,
            id,
            on_submit: Box::new(on_submit),
        }
    }
}

impl Render for Input {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id(self.id)
            .on_click(cx.listener(|this, _event, window, cx| {
                this.focus_handle.focus(window);
                cx.notify();
            }))
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                if event.keystroke.key == "enter" {
                    (this.on_submit)(&this.value, cx);
                } else if let Some(key) = &event.keystroke.key_char {
                    this.value += key;
                    this.cursor += key.len();
                } else if event.keystroke.key == "backspace" {
                    if let Some(_) = this.value.pop() {
                        this.cursor -= 1;
                    }
                } else if event.keystroke.key == "left" {
                    if this.cursor > 0 {
                        this.cursor -= 1;
                    }
                } else if event.keystroke.key == "right" {
                    if this.cursor < this.value.len() {
                        this.cursor += 1;
                    }
                }
                cx.notify();
            }))
            .w_full()
            .h_10()
            .px_3()
            .rounded_sm()
            .flex()
            .items_center()
            .bg(rgb(theme::colours::WINTER))
            .cursor_text()
            .child(self.value.clone())
            .when(self.focus_handle.is_focused(window), |this| {
                this.child("|")
            })
    }
}
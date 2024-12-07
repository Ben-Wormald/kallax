use gpui::*;

use crate::*;
use elements::UiAction;

type Vcx<'a> = ViewContext<'a, Shelves>;

pub struct Shelves {
    shelves: Vec<Shelf>,
}

impl Shelves {
    pub fn new(cx: &mut Vcx, library: &Model<Library>) -> Shelves {
        let shelves = Vec::new();

        Shelves {
            shelves
        }
    }
}

impl Render for Shelves {
    fn render(&mut self, cx: &mut Vcx) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .min_h_0()
            .child(String::from("SHELVES"))
            .child(
                div()
                    .children(
                        self.shelves.iter().map(|shelf|
                            div().child(shelf.name().to_string())
                        )
                    )
            )
    }
}

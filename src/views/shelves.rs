use gpui::*;
use std::sync::Arc;

use crate::{KallaxEntity, Library, UiEvent};

type Vcx<'a> = ViewContext<'a, Shelves>;

pub struct Shelves {
    shelves: Vec<KallaxEntity>,
}

impl Shelves {
    pub fn new(cx: &mut Vcx) -> Shelves {
        let searches = cx.global::<Library>().searches.iter().map(|search| KallaxEntity::Search(Arc::clone(search)));
        let playlists = cx.global::<Library>().playlists.iter().map(|playlist| KallaxEntity::Playlist(Arc::clone(playlist)));

        let shelves = searches.chain(playlists).collect();

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
                        self.shelves.iter().map(|shelf| {
                            let shelf_id = shelf.id();
                            div()
                                .id(ElementId::Name(shelf_id.clone().into()))
                                .child(shelf.name().to_string())
                                .on_click(cx.listener(move |_this, _event, cx| {
                                    cx.emit(Arc::new(UiEvent::EntityOpened(shelf_id.clone())))
                                }))
                        })
                    )
            )
    }
}

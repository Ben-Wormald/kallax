use gpui::*;
use std::sync::Arc;

use crate::*;

type Vcx<'a> = ViewContext<'a, Browse>;

enum ItemsMode {
    List,
    Grid,
    // CoverFlow,
}

pub struct Browse {
    items_mode: ItemsMode,
    entity: Option<KallaxEntity>,
    entities: Vec<KallaxEntity>,
}

impl Browse {
    pub fn new(_cx: &mut Vcx) -> Browse {
        // let tracks = cx.new_view(|cx| Tracks::new(cx));
        // let albums = cx.new_view(|cx| Albums::new(cx));

        Browse {
            items_mode: ItemsMode::Grid,
            entity: None,
            entities: Vec::new(),
        }
    }

    pub fn set_entity(&mut self, cx: &mut Vcx, entity_id: String) {
        self.entity = cx.global::<Library>().get_entity(&entity_id);
        self.entities = match &self.entity {
            Some(KallaxEntity::Search(search)) => cx.global::<Library>().execute_search(&search.id()),
            _ => todo!(),
        }
    }

    pub fn open_album(&mut self, _cx: &mut Vcx, _album: &Arc<Album>) {
        // self.tracks.update(cx, |this, cx| {
        //     this.update_view(
        //         cx,
        //         library,
        //         tracks::TrackView::Album(album.artist_name.clone(), album.title.clone()),
        //     );
        // });
    }
}

impl Render for Browse {
    fn render(&mut self, cx: &mut Vcx) -> impl IntoElement {
        let header = div()
            .id("browse-header");

        let header = match &self.entity {
            Some(KallaxEntity::Album(album)) => header.child(album.title.clone()),
            Some(KallaxEntity::Search(search)) => header.child(search.name.clone()),
            None => header.child(String::from("welcome")),
            _ => unimplemented!(),
        };
        
        let items = div()
            .id("browse-items");

        let items = match self.items_mode {
            ItemsMode::Grid => items.children(
                self.entities.iter().map(|e| {
                    let e = e.clone();
                    div()
                        .id(ElementId::Name(e.id().into()))
                        .child(e.name().to_string())
                        .on_click(cx.listener(move |_this, _event, cx| {
                            on_click_entity(cx, &e);
                        }))
                })
            ),
            ItemsMode::List => items.children(
                self.entities.iter().map(|e| {
                    let e = e.clone();
                    div()
                        .id(ElementId::Name(e.id().into()))
                        .child(e.name().to_string())
                        .on_click(cx.listener(move |_this, _event, cx| {
                            on_click_entity(cx, &e);
                        }))
                })
            ),
        };

        div()
            .flex_grow()
            .flex()
            .flex_col()
            .min_h_0()
            .child(header)
            .child(items)
    }
}

fn on_click_entity(cx: &mut Vcx, entity: &KallaxEntity) {
    match entity {
        KallaxEntity::Track(track) => cx.emit(UiEvent::play(track)),
        _ => todo!(),
    }
    cx.notify();
}

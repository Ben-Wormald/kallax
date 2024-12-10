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
    fn render(&mut self, _cx: &mut Vcx) -> impl IntoElement {
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
            ItemsMode::Grid => items.child(div().children(self.entities.iter().map(|e| div().child(e.name().to_string())))),
            ItemsMode::List => items.child(div().children(self.entities.iter().map(|e| div().child(e.name().to_string())))),
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

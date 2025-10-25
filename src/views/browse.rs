use elements::list_entity;
use gpui::*;
use std::sync::Arc;

use crate::{elements::Input, *};

type Vcx<'a> = Context<'a, Browse>;

enum ItemsMode {
    List,
    Grid,
    // CoverFlow,
}

pub enum BrowseContext {
    Album(usize),
    Artist,
    Search,
    Playlist(usize),
}

pub struct Browse {
    items_mode: ItemsMode,
    entity: Option<KallaxEntity>,
    entities: Vec<KallaxEntity>,
    list_state: ListState,
    search: Entity<Input>,
}

impl Browse {
    pub fn new(cx: &mut Vcx) -> Browse {
        // let tracks = cx.new(|cx| Tracks::new(cx));
        // let albums = cx.new(|cx| Albums::new(cx));

        let search = cx.new(|cx| Input::new("browse-search", cx));

        Browse {
            items_mode: ItemsMode::List,
            entity: None,
            entities: Vec::new(),
            list_state: ListState::new(0, ListAlignment::Top, px(200.0)),
            search,
        }
    }

    pub fn set_entity(&mut self, cx: &mut Vcx, entity_id: String) {
        self.entity = cx.global::<Library>().get_entity(&entity_id);
        self.entities = match &self.entity {
            Some(KallaxEntity::Search(search)) => cx.global::<Library>().execute_search(&search.id()),
            Some(KallaxEntity::Album(album)) => cx.global::<Library>().get_tracks(&album.track_ids),
            _ => todo!(),
        };
        self.list_state =  ListState::new(self.entities.len(), ListAlignment::Top, px(200.0));
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

    pub fn render_entity(&mut self, index: usize, _window: &mut Window, cx: &mut Vcx) -> AnyElement {
        let entity = self.entities.get(index).unwrap();

        match self.items_mode {
            ItemsMode::Grid => {
                let entity = entity.clone();
                div()
                    .id(ElementId::Name(entity.id().into()))
                    .child(entity.name().to_string())
                    .on_click(cx.listener(move |_this, _event, _window, cx| {
                        on_click_entity(cx, &entity);
                    }))
                    .into_any_element()
            },
            ItemsMode::List => {
                let browse_context = match &self.entity {
                    Some(KallaxEntity::Album(_)) => BrowseContext::Album(index),
                    Some(KallaxEntity::Search(_)) => BrowseContext::Search,
                    Some(KallaxEntity::Artist(_)) => BrowseContext::Artist,
                    Some(KallaxEntity::Playlist(_)) => BrowseContext::Playlist(index),
                    _ => unimplemented!(),
                };
                list_entity(entity, browse_context, cx).into_any_element()
            }
        }
    }
}

impl Render for Browse {
    fn render(&mut self, _window: &mut Window, cx: &mut Vcx) -> impl IntoElement {
        let header = div()
            .id("browse-header");

        let header = match &self.entity {
            Some(KallaxEntity::Album(album)) => header.child(album.title.clone()),
            Some(KallaxEntity::Search(search)) => header.child(search.name.clone()),
            None => header.child(String::from("welcome")),
            _ => unimplemented!(),
        };

        div()
            .flex_grow()
            .flex()
            .flex_col()
            .h_full()
            .child(self.search.clone())
            .child(header)
            .child(
                list(
                    self.list_state.clone(),
                    cx.processor(Self::render_entity),
                )
                .h_full()
                .w_full()
            )
    }
}

fn on_click_entity(cx: &mut Vcx, entity: &KallaxEntity) {
    match entity {
        KallaxEntity::Track(track) => cx.emit(UiEvent::play(track)),
        _ => todo!(),
    }
    cx.notify();
}

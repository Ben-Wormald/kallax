use gpui::*;
use std::sync::Arc;

use crate::*;

use context_menu::ContextMenuItem;

pub struct UiAction {
    pub label: &'static str,
    pub event: Arc<UiEvent>,
}

pub enum BrowseContext {
    Album(usize),
    Artist,
    Search,
    Playlist(usize),
}

pub fn list_entity(
    entity: &KallaxEntity,
    browse_context: BrowseContext,
    cx: &mut ViewContext<Browse>,
) -> impl IntoElement {
    let children = match browse_context {
        BrowseContext::Album(track_number) => vec![
            div().child(track_number.to_string()),
            div().child(entity.name().to_string()),
        ],
        BrowseContext::Artist => match entity {
            KallaxEntity::Album(_) => vec![
                div().child(entity.name().to_string()),
            ],
            KallaxEntity::Track(track) => vec![
                div().child(entity.name().to_string()),
                div().child(cx.global::<Library>()
                    .get_album(&track.album_id)
                    .map_or(String::from(""), |album| album.title.clone())
                ),
            ],
            _ => unimplemented!(),
        },
        BrowseContext::Search => vec![],
        BrowseContext::Playlist(_track_number) => vec![],
    };

    let id = ElementId::Name(entity.id().into());

    let entity = entity.clone();
    let on_click = cx.listener(move |_this, _event, cx| {
        match &entity {
            KallaxEntity::Track(track) => cx.emit(UiEvent::play(&track)),
            _ => todo!(),
        }
        cx.notify();
    });

    div()
        .id(id)
        .on_click(on_click)
        .children(children)
}

pub fn track(track: &Arc<Track>, cx: &mut ViewContext<Tracks>) -> impl IntoElement {
    let on_click = cx.listener({
        let track = Arc::clone(track);
        move |_this, _event, cx| cx.emit(UiEvent::play(&track))
    });
    
    let on_right_click = cx.listener({
        let track = Arc::clone(track);
        move |_this, event: &MouseDownEvent, cx: &mut ViewContext<Tracks>| {
            cx.emit(Arc::new(UiEvent::RightClick(RightClickEvent {
                position: event.position,
                items: Arc::new(vec![
                    ContextMenuItem {
                        label: "Play",
                        event: UiEvent::play(&track),
                    },
                    ContextMenuItem {
                        label: "Queue",
                        event: UiEvent::queue(&track),
                    },
                ]),
            })));
        }
    });

    div()
        .id(ElementId::Name(track.title.clone().into()))
        .flex()
        .gap_3()
        .h_8()
        .items_center()
        .rounded(px(1.))
        .hover(|style| style.bg(rgb(theme::colours::TOUCH)))
        .child(
            div()
                .flex()
                .w_8()
                .justify_end()
                .text_color(rgb(theme::colours::YOUTH))
                .child(track.track_number.map_or(
                    String::from(""),
                    |track_number| track_number.to_string(),
                ))
        )
        .child(
            div().child(track.title.clone())
        )
        .on_click(on_click)
        .on_mouse_down(MouseButton::Right, on_right_click)
}

pub fn album(album: &Arc<Album>, cx: &mut ViewContext<Albums>) -> impl IntoElement {
    let element = div()
        .id(ElementId::Name(album.id().into()))
        .size_64();

    let element = if let Some(artwork) = &album.artwork {
        element.child(
            img(Arc::clone(artwork))
                .rounded(px(1.))
                .size_full()
        )
    } else {
        element
    };

    element
        .on_click({
            let album = Arc::clone(album);
            cx.listener(move |_this, _event, cx| {
                cx.emit(UiEvent::album(&album));
            })
        })
}

pub fn tab_bar<V: EventEmitter<Arc<UiEvent>>>(
    tabs: Vec<UiAction>,
    selected: usize,
    cx: &mut ViewContext<V>,
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
                .child(item.label);

            if index == selected {
                tab
                    .bg(rgb(theme::colours::AMSTERDAM))
                    .text_color(rgb(theme::colours::WINTER))
                    .rounded_t_sm()
            } else {
                tab
                    .text_color(rgb(theme::colours::AMSTERDAM))
                    .rounded_t_sm()
                    .hover(|style| style
                        .rounded_sm()
                        .bg(rgb(theme::colours::YOUTH))
                        .text_color(rgb(theme::colours::SHALLOWS))
                    ).on_click(cx.listener(move |_this, _event, cx| {
                        cx.emit(Arc::clone(&item.event));
                    }))
            }
        }))
}

use std::sync::Arc;

use gpui::{prelude::FluentBuilder, *};

use crate::{events::RightClickEvent, views::context_menu::ContextMenuItem};

use super::{browse::BrowseContext, theme, Browse, KallaxEntity, Library, UiEvent};

pub fn list_entity(
    entity: &KallaxEntity,
    browse_context: BrowseContext,
    index: usize,
    cx: &mut Context<Browse>,
) -> impl IntoElement {
    let children = match browse_context {
        BrowseContext::Album(track_number) => vec![
            row_column(true)
                .child(track_number.to_string()),
            row_column(true)
                .child(entity.name().to_string()),
        ],
        BrowseContext::Artist => match entity {
            KallaxEntity::Album(album) => vec![
                row_column(true)
                    .child(album.title.clone()),
            ],
            KallaxEntity::Track(track) => vec![
                row_column(true)
                    .child(track.title.clone()),
                row_column(true)
                    .child(cx.global::<Library>()
                        .get_album(&track.album_id)
                        .map_or(String::from(""), |album| album.name().to_string())
                    ),
            ],
            _ => unimplemented!(),
        },
        BrowseContext::Search => match entity {
            KallaxEntity::Artist(artist) => vec![
                row_column(true)
                    .child(artist.name.clone()),
            ],
            KallaxEntity::Album(album) => vec![
                row_column(true)
                    .child(album.title.clone()),
                row_column(true)
                    .child(cx.global::<Library>()
                        .get_artist(&album.artist_id)
                        .map_or(String::from(""), |artist| artist.name().to_string())
                    ),
            ],
            KallaxEntity::Track(track) => vec![
                row_column(true)
                    .child(track.title.clone()),
                row_column(true)
                    .child(cx.global::<Library>()
                        .get_album(&track.album_id)
                        .map_or(String::from(""), |album| album.name().to_string())
                    ),
                row_column(true)
                    .child(cx.global::<Library>()
                        .get_artist(&track.artist_id)
                        .map_or(String::from(""), |artist| artist.name().to_string())
                    ),
            ],
            _ => unimplemented!(),
        },
        BrowseContext::Playlist(track_number) => vec![
            row_column(false)
                .text_align(TextAlign::Right)
                .child(track_number.to_string()),
            row_column(true)
                .child(entity.name().to_string()),
        ],
    };

    let id = ElementId::Name(entity.id().into());

    let click_entity = entity.clone();
    let on_click = cx.listener(move |this, _event, _window, cx| {
        match &click_entity {
            KallaxEntity::Track(track) => {
                cx.emit(UiEvent::play(&this.entities, index))
            },
            entity => cx.emit(Arc::new(UiEvent::EntityOpened(entity.id()))),
        }
        cx.notify();
    });

    let right_click_entity = entity.clone();
    let on_right_click = cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
        match &right_click_entity {
            KallaxEntity::Track(track) => {
                cx.emit(Arc::new(UiEvent::RightClick(RightClickEvent {
                    position: event.position,
                    items: Arc::new(vec![
                        ContextMenuItem {
                            label: "Play",
                            event: UiEvent::play(&this.entities, index),
                        },
                        ContextMenuItem {
                            label: "Queue",
                            event: UiEvent::queue(&track),
                        },
                    ]),
                })));
            },
            _ => unimplemented!(),
        }
        cx.notify();
    });

    div()
        .id(id)
        .on_click(on_click)
        .on_mouse_down(MouseButton::Right, on_right_click)
        .w_full()
        .flex()
        .rounded_md()
        .hover(|s| s.bg(rgb(theme::colours::SMOTHER)))
        .children(children)
}

fn row_column(grow: bool) -> Div {
    div()
        .when(grow, |s| s.flex_grow().flex_basis(px(1.)))
        .py_0p5()
        .px_2()
        .hover(|s| s.text_decoration_solid().text_decoration_1())
}

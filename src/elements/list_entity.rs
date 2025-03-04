use std::sync::Arc;

use gpui::*;

use super::{browse::BrowseContext, theme, Browse, KallaxEntity, Library, UiEvent};

pub fn list_entity(
    entity: &KallaxEntity,
    browse_context: BrowseContext,
    cx: &mut Context<Browse>,
) -> impl IntoElement {
    let children = match browse_context {
        BrowseContext::Album(track_number) => vec![
            row_column()
                .child(track_number.to_string()),
            row_column()
                .child(entity.name().to_string()),
        ],
        BrowseContext::Artist => match entity {
            KallaxEntity::Album(album) => vec![
                row_column()
                    .child(album.title.clone()),
            ],
            KallaxEntity::Track(track) => vec![
                row_column()
                    .child(track.title.clone()),
                row_column()
                    .child(cx.global::<Library>()
                        .get_album(&track.album_id)
                        .map_or(String::from(""), |album| album.name().to_string())
                    ),
            ],
            _ => unimplemented!(),
        },
        BrowseContext::Search => match entity {
            KallaxEntity::Artist(artist) => vec![
                row_column()
                    .child(artist.name.clone()),
            ],
            KallaxEntity::Album(album) => vec![
                row_column()
                    .child(album.title.clone()),
                row_column()
                    .child(cx.global::<Library>()
                        .get_artist(&album.artist_id)
                        .map_or(String::from(""), |artist| artist.name().to_string())
                    ),
            ],
            KallaxEntity::Track(track) => vec![
                row_column()
                    .child(track.title.clone()),
                row_column()
                    .child(cx.global::<Library>()
                        .get_album(&track.album_id)
                        .map_or(String::from(""), |album| album.name().to_string())
                    ),
                row_column()
                    .child(cx.global::<Library>()
                        .get_artist(&track.artist_id)
                        .map_or(String::from(""), |artist| artist.name().to_string())
                    ),
            ],
            _ => unimplemented!(),
        },
        BrowseContext::Playlist(track_number) => vec![
            row_column()
                .child(track_number.to_string()),
            row_column()
                .child(entity.name().to_string()),
        ],
    };

    let id = ElementId::Name(entity.id().into());

    let entity = entity.clone();
    let on_click = cx.listener(move |_this, _event, _window, cx| {
        match &entity {
            KallaxEntity::Track(track) => cx.emit(UiEvent::play(&track)),
            entity => cx.emit(Arc::new(UiEvent::EntityOpened(entity.id()))),
        }
        cx.notify();
    });

    div()
        .id(id)
        .on_click(on_click)
        .flex()
        .children(children)
        .hover(|s| s.bg(rgb(theme::colours::SMOTHER)))
}

fn row_column() -> Div {
    div()
        .flex_grow()
        .flex_basis(px(1.))
        .py_0p5()
        .px_2()
        .hover(|s| s.text_decoration_solid())
}

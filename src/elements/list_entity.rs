use gpui::*;

use super::{browse::BrowseContext, Browse, KallaxEntity, Library, UiEvent};

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
            KallaxEntity::Album(album) => vec![
                div().child(album.title.clone()),
            ],
            KallaxEntity::Track(track) => vec![
                div().child(track.title.clone()),
                div().child(cx.global::<Library>()
                    .get_album(&track.album_id)
                    .map_or(String::from(""), |album| album.title.clone())
                ),
            ],
            _ => unimplemented!(),
        },
        BrowseContext::Search => match entity {
            KallaxEntity::Artist(artist) => vec![
                div().child(artist.name.clone()),
            ],
            KallaxEntity::Album(album) => vec![
                div().child(album.title.clone()),
                div().child(cx.global::<Library>()
                    .get_artist(&album.artist_id)
                    .map_or(String::from(""), |artist| artist.name.clone())
                ),
            ],
            KallaxEntity::Track(track) => vec![
                div().child(track.title.clone()),
                div().child(cx.global::<Library>()
                    .get_album(&track.album_id)
                    .map_or(String::from(""), |album| album.title.clone())
                ),
                div().child(cx.global::<Library>()
                    .get_artist(&track.artist_id)
                    .map_or(String::from(""), |artist| artist.name.clone())
                ),
            ],
            _ => unimplemented!(),
        },
        BrowseContext::Playlist(track_number) => vec![
            div().child(track_number.to_string()),
            div().child(entity.name().to_string()),
        ],
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
        .flex()
        .children(children)
}
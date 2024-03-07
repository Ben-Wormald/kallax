use gpui::*;
use std::{collections::HashSet, sync::Arc};

use crate::*;

type Vcx<'a> = ViewContext<'a, Albums>;

pub struct Albums {
    pub view: AlbumView,
    albums: Vec<Arc<Album>>,
}

pub enum AlbumView {
    AllAlbums,
    ArtistAlbums(String),
}

impl Albums {
    pub fn new(cx: &mut Vcx, library: &Model<Library>) -> Albums {
        cx.observe(library, |this, library, cx| {
            this.albums = get_albums(cx, &library, &this.view);
            cx.notify();
        }).detach();

        let view = AlbumView::AllAlbums;
        let albums = get_albums(cx, library, &view);

        Albums {
            view,
            albums,
        }
    }
}

impl Render for Albums {
    fn render(&mut self, cx: &mut ViewContext<Albums>) -> impl IntoElement {
        div()
            .id("albums")
            .flex()
            .flex_wrap()
            .gap(px(1.))
            .children(
                self.albums.iter().map(|album|
                    elements::album(album, cx)
                )
            )
    }
}

fn get_albums(cx: &mut Vcx, library: &Model<Library>, view: &AlbumView) -> Vec<Arc<Album>> {
    let tracks = (*library.read(cx).tracks).clone();
    let mut albums: HashSet<Album> = HashSet::new();

    match view {
        AlbumView::AllAlbums => tracks.iter()
            .for_each(|track| get_album(track, &mut albums)),
        AlbumView::ArtistAlbums(artist) => tracks.iter()
            .filter(|track| track.artist_name == *artist)
            .for_each(|track| get_album(track, &mut albums)),
    };

    albums.into_iter().map(|album| Arc::new(album)).collect()
}

fn get_album(track: &Arc<Track>, albums: &mut HashSet<Album>) {
    let mut album = Album {
        title: track.album_title.clone(),
        artist_name: track.artist_name.clone(),
        duration: track.duration.unwrap_or(0),
        artwork: track.artwork.clone(),
    };
    
    if let Some(existing) = albums.get(&album) {
        album.duration += existing.duration;
    }

    albums.insert(album);
}

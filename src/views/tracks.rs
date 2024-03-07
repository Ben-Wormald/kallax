use gpui::*;
use std::sync::Arc;

use crate::*;

use self::elements::UiAction;

type Vcx<'a> = ViewContext<'a, Tracks>;

pub struct Tracks {
    pub view: TrackView,
    tracks: Vec<Arc<Track>>,
    sort_dropdown: View<Dropdown>,
}

pub enum TrackView {
    AllTracks,
    // AllArtists,
    // AllAlbums,
    ArtistTracks(String),
    // ArtistAlbums(String),
    Album(String, String),
    // Label(String),
    // Playlist(String),
}

impl Tracks {
    pub fn new(cx: &mut Vcx, library: &Model<Library>) -> Tracks {
        cx.observe(library, |this, library, cx| {
            this.tracks = get_tracks(cx, &library, &this.view);
            cx.notify();
        }).detach();

        let view = TrackView::AllTracks;
        let tracks = get_tracks(cx, library, &view);
        let sort_dropdown = cx.new_view(|_cx| Dropdown::new("Sort", vec![
            UiAction { label: "Order", event: Arc::new(UiEvent::PauseClicked) }
        ]));

        Tracks {
            view,
            tracks,
            sort_dropdown,
        }
    }

    pub fn update_view(&mut self, cx: &mut Vcx, library: &Model<Library>, view: TrackView) {
        self.view = view;
        self.tracks = get_tracks(cx, library, &self.view);
    }
}

impl Render for Tracks {
    fn render(&mut self, cx: &mut ViewContext<Tracks>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(1.))
            .child(
                div()
                    .flex()
                    .child(self.sort_dropdown.clone())
            )
            .children(
                self.tracks.iter().enumerate().map(|(index, track)|
                    elements::track(index, track, cx)
                )
            )
    }
}

fn get_tracks(cx: &mut Vcx, library: &Model<Library>, view: &TrackView) -> Vec<Arc<Track>> {
    let tracks = (*library.read(cx).tracks).clone();

    match view {
        TrackView::AllTracks => tracks,
        TrackView::ArtistTracks(artist) => tracks.into_iter()
            .filter(|track| track.artist_name == *artist).collect(),
        TrackView::Album(artist, album) => tracks.into_iter()
            .filter(|track| track.artist_name == *artist && track.album_title == *album).collect(),
    }
}

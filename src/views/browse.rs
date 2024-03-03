use gpui::*;
use std::sync::Arc;

use crate::*;
use elements::{tab_bar, TabBarItem};

type Vcx<'a> = ViewContext<'a, Browse>;

const TRACKS: usize = 0;
const ARTISTS: usize = 1;
const ALBUMS: usize = 2;
const PLAYLISTS: usize = 3;

pub struct Browse {
    pub selected_tab: usize,
    pub tracks: View<Tracks>,
}

impl Browse {
    pub fn new(cx: &mut Vcx, library: &Model<Library>) -> Browse {
        let tracks = cx.new_view(|cx| Tracks::new(cx, library));

        Browse {
            selected_tab: TRACKS,
            tracks,
        }
    }
}

impl Render for Browse {
    fn render(&mut self, cx: &mut Vcx) -> impl IntoElement {
        let browse = div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                tab_bar(vec![
                    TabBarItem {
                        label: "Tracks",
                        event: Arc::new(UiEvent::BrowseTabClicked(TRACKS)),
                    },
                    TabBarItem {
                        label: "Artists",
                        event: Arc::new(UiEvent::BrowseTabClicked(ARTISTS)),
                    },
                    TabBarItem {
                        label: "Albums",
                        event: Arc::new(UiEvent::BrowseTabClicked(ALBUMS)),
                    },
                    TabBarItem {
                        label: "Playlists",
                        event: Arc::new(UiEvent::BrowseTabClicked(PLAYLISTS)),
                    },
                ], self.selected_tab, cx)
            );

        match self.selected_tab {
            TRACKS => browse.child(self.tracks.clone()),
            _ => browse,
        }
    }
}

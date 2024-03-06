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
    pub albums: View<Albums>,
}

impl Browse {
    pub fn new(cx: &mut Vcx, library: &Model<Library>) -> Browse {
        let tracks = cx.new_view(|cx| Tracks::new(cx, library));
        let albums = cx.new_view(|cx| Albums::new(cx, library));

        Browse {
            selected_tab: TRACKS,
            tracks,
            albums,
        }
    }
}

impl Render for Browse {
    fn render(&mut self, cx: &mut Vcx) -> impl IntoElement {
        let view = div()
            .id("browse-view")
            .flex_grow()
            .overflow_scroll()
            .rounded_b_sm()
            .bg(rgb(theme::colours::AMSTERDAM))
            .p(px(1.));

        let view = match self.selected_tab {
            TRACKS => view.child(self.tracks.clone()),
            ALBUMS => view.child(self.albums.clone()),
            _ => view,
        };

        let view = if self.selected_tab != 0 { view.rounded_tl_sm() } else { view };
        let view = if self.selected_tab != 3 { view.rounded_tr_sm() } else { view };

        div()
            .flex_grow()
            .flex()
            .flex_col()
            .min_h_0()
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
            )
            .child(view)
    }
}

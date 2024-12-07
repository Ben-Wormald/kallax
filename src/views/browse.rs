use gpui::*;
use std::sync::Arc;

use crate::*;
use elements::UiAction;

type Vcx<'a> = ViewContext<'a, Browse>;

enum HeaderMode {
    Home,
    Search,
    Playlist,
    Album,
    Artist,
    Label,
}

enum ItemsMode {
    List,
    Grid,
    // CoverFlow,
}

pub struct Browse {
    pub header_mode: HeaderMode,
    pub items_mode: ItemsMode,
    // pub tracks: View<Tracks>,
    // pub albums: View<Albums>,
}

impl Browse {
    pub fn new(cx: &mut Vcx, library: &Model<Library>) -> Browse {
        let tracks = cx.new_view(|cx| Tracks::new(cx, library));
        let albums = cx.new_view(|cx| Albums::new(cx, library));

        Browse {
            header_mode: HeaderMode::Home,
            items_mode: ItemsMode::Grid,
        }
    }

    pub fn open_album(&mut self, cx: &mut Vcx, library: &Model<Library>, album: &Arc<Album>) {
        self.header_mode = HeaderMode::Album;
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
    fn render(&mut self, cx: &mut Vcx) -> impl IntoElement {
        let header = div()
            .id("browse-header");

        let header = match self.header_mode {
            HeaderMode::Home => header.child(String::from("hi")),
            _ => header.child(div()),
        };
        
        let items = div()
            .id("browse-items");

        let items = match self.items_mode {
            ItemsMode::Grid => items.child(String::from("hi")),
            _ => items.child(div()),
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

use gpui::*;
use std::sync::Arc;

use crate::*;

use self::tracks::TrackView;

pub struct Kallax {
    playback: Model<Playback>,
    _scrobbler: Model<Scrobbler>,
    library: Model<Library>,
    browse: View<Browse>,
    now_playing: View<NowPlaying>,
    context_menu: View<ContextMenu>,
    modal: View<Modal>,
}

impl Kallax {
    pub fn new(cx: &mut ViewContext<Kallax>) -> Kallax {
        let playback = cx.new_model(Playback::new);
        let _scrobbler = cx.new_model(|cx| Scrobbler::new(cx, &playback));
        let library = cx.new_model(Library::new);

        let browse = cx.new_view(|cx| Browse::new(cx, &library));
        let now_playing = cx.new_view(|cx| NowPlaying::new(cx, &playback));
        let context_menu = cx.new_view(|_cx| ContextMenu::new());
        let modal = cx.new_view(|_cx| Modal::new());

        cx.subscribe(&browse, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        let tracks = browse.read(cx).tracks.clone();
        cx.subscribe(&tracks, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        let albums = browse.read(cx).albums.clone();
        cx.subscribe(&albums, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        cx.subscribe(&context_menu, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        cx.subscribe(&now_playing, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        Kallax {
            playback,
            _scrobbler,
            library,
            browse,
            now_playing,
            context_menu,
            modal,
        }
    }

    pub fn handle_ui_event(&mut self, event: &Arc<UiEvent>, cx: &mut ViewContext<Kallax>) {
        match (**event).clone() {
            UiEvent::PlayClicked(event) => self.playback.update(cx, |this, cx| {
                this.play(Arc::clone(&event.track), cx);
                cx.notify();
            }),
            UiEvent::QueueClicked(event) => self.playback.update(cx, |this, cx| {
                this.add_to_queue(Arc::clone(&event.track), cx);
                cx.notify();
            }),
            UiEvent::PauseClicked => self.playback.update(cx, |this, cx| {
                this.pause(cx);
                cx.notify();
            }),
            UiEvent::ResumeClicked => self.playback.update(cx, |this, cx| {
                this.resume(cx);
                cx.notify();
            }),
            UiEvent::SkipClicked => self.playback.update(cx, |this, cx| {
                this.skip(cx);
                cx.notify();
            }),
            UiEvent::AlbumClicked(album) => self.browse.update(cx, |this, cx| {
                this.open_album(cx, &self.library, &album);
                cx.notify();
            }),
            UiEvent::NowPlayingTabClicked(tab_index) => self.now_playing.update(cx, |this, cx| {
                this.selected_tab = tab_index;
                cx.notify();
            }),
            UiEvent::BrowseTabClicked(tab_index) => self.browse.update(cx, |this, cx| {
                this.selected_tab = tab_index;
                cx.notify();
            }),
            UiEvent::RightClick(event) => self.context_menu.update(cx, |this, cx| {
                this.items = Arc::clone(&event.items);
                this.position = Some(event.position);
                cx.notify();
            }),
        };
    }
}

impl Render for Kallax {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .min_h_0()
            .p_1()
            .child(
                div()
                    .min_h(px(30.))
            )
            .child(
                div()
                    .flex_grow()
                    .flex()
                    .min_h_0()
                    .gap(px(2.))
                    .bg(rgb(theme::colours::STILL))
                    .rounded_md()
                    .p(px(2.))
                    .text_color(rgb(theme::colours::WINTER))
                    .font("Iosevka")
                    .child(self.browse.clone())
                    .child(self.now_playing.clone())
                    .child(self.context_menu.clone())
                    .child(self.modal.clone())
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, cx| {
                        this.context_menu.update(cx, |context_menu, _cx| {
                            context_menu.position = None;
                        });
                    }))
            )
    }
}

use gpui::*;
use std::sync::Arc;

use crate::*;

pub struct Kallax {
    playback: Entity<Playback>,
    _scrobbler: Entity<Scrobbler>,
    macos: Entity<MacOS>,
    shelves: Entity<Shelves>,
    browse: Entity<Browse>,
    now_playing: Entity<NowPlaying>,
    context_menu: Entity<ContextMenu>,
    modal: Entity<Modal>,
    focus_handle: FocusHandle,
}

impl Kallax {
    pub fn new(window: &mut Window, cx: &mut Context<Kallax>) -> Kallax {
        cx.set_global(Library::new());

        let playback = cx.new(Playback::new);
        let _scrobbler = cx.new(|cx| Scrobbler::new(cx, &playback));
        let macos = cx.new(|cx| MacOS::new(cx, &playback));

        playback.update(cx, |_this, cx| {
            cx.subscribe(&macos, move |subscriber, _emitter, event: &Arc<PlaybackEvent>, cx| {
                subscriber.handle_event(event, cx);
            }).detach();
        });

        let shelves = cx.new(Shelves::new);
        let browse = cx.new(Browse::new);
        let now_playing = cx.new(|cx| NowPlaying::new(cx, &playback));
        let context_menu = cx.new(|_cx| ContextMenu::new());
        let modal = cx.new(|_cx| Modal::new());

        cx.subscribe(&shelves, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        cx.subscribe(&browse, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        // let tracks = browse.read(cx).tracks.clone();
        // cx.subscribe(&tracks, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
        //     subscriber.handle_ui_event(event, cx);
        // }).detach();

        // let albums = browse.read(cx).albums.clone();
        // cx.subscribe(&albums, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
        //     subscriber.handle_ui_event(event, cx);
        // }).detach();

        cx.subscribe(&context_menu, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        cx.subscribe(&now_playing, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        let focus_handle = cx.focus_handle();
        window.focus(&focus_handle);

        Kallax {
            playback,
            _scrobbler,
            macos,
            shelves,
            browse,
            now_playing,
            context_menu,
            modal,
            focus_handle,
        }
    }

    pub fn handle_ui_event(&mut self, event: &Arc<UiEvent>, cx: &mut Context<Kallax>) {
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
                this.open_album(cx, &album);
                cx.notify();
            }),
            UiEvent::NowPlayingTabClicked(tab_index) => self.now_playing.update(cx, |this, cx| {
                this.selected_tab = tab_index;
                cx.notify();
            }),
            UiEvent::RightClick(event) => self.context_menu.update(cx, |this, cx| {
                this.items = Arc::clone(&event.items);
                this.position = Some(event.position);
                cx.notify();
            }),
            UiEvent::EntityOpened(entity_id) => self.browse.update(cx, |this, cx| {
                dbg!(&entity_id);
                this.set_entity(cx, entity_id);
                cx.notify();
            }),
        };
    }
}

impl Render for Kallax {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .min_h_0()
            .bg(rgb(theme::colours::SMOTHER))
            .text_color(rgb(theme::colours::SHALLOWS))
            .font(Font {
                family: "Work Sans".into(),
                features: FontFeatures(Arc::new(Vec::new())),
                fallbacks: None,
                weight: FontWeight::NORMAL,
                style: FontStyle::Normal,
            })
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                this.context_menu.update(cx, |context_menu, _cx| {
                    context_menu.position = None;
                });
            }))
            .on_action(cx.listener(|this, _: &ShelfOne, _window, cx| {
                if let Some(shelf) = this.shelves.read(cx).shelves.first() {
                    let shelf_id = shelf.id();
                    this.browse.update(cx, move |this, cx| {
                        this.set_entity(cx, shelf_id);
                        cx.notify();
                    });
                }
            }))
            .child(
                div()
                    .min_h(px(30.)) // title bar
            )
            .child(
                div()
                    .flex_grow()
                    .flex()
                    .min_h_0()
                    .child(self.shelves.clone())
                    .child(self.browse.clone())
                    .child(self.now_playing.clone())
            )
            .child(
                div()
                    .min_h(px(30.)) // seek
            )
            .child(self.context_menu.clone())
            .child(self.modal.clone())
    }
}

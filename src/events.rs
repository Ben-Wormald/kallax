use std::sync::Arc;

use crate::*;

use context_menu::ContextMenuItem;

#[derive(Clone)]
pub enum UiEvent {
    PlayClicked(PlayClickedEvent),
    QueueClicked(QueueClickedEvent),
    PauseClicked,
    ResumeClicked,
    SkipClicked,
    AlbumClicked(Arc<Album>),
    NowPlayingTabClicked(usize),
    BrowseTabClicked(usize),
    RightClick(RightClickEvent),
}
impl UiEvent {
    pub fn play(track: &Arc<Track>) -> Arc<UiEvent> {
        Arc::new(UiEvent::PlayClicked(PlayClickedEvent { track: Arc::clone(track) }))
    }

    pub fn queue(track: &Arc<Track>) -> Arc<UiEvent> {
        Arc::new(UiEvent::QueueClicked(QueueClickedEvent { track: Arc::clone(track) }))
    }

    pub fn album(album: &Arc<Album>) -> Arc<UiEvent> {
        Arc::new(UiEvent::AlbumClicked(Arc::clone(album)))
    }
}
impl gpui::EventEmitter<Arc<UiEvent>> for Albums {}
impl gpui::EventEmitter<Arc<UiEvent>> for Browse {}
impl gpui::EventEmitter<Arc<UiEvent>> for ContextMenu {}
impl gpui::EventEmitter<Arc<UiEvent>> for Dropdown {}
impl gpui::EventEmitter<Arc<UiEvent>> for NowPlaying {}
impl gpui::EventEmitter<Arc<UiEvent>> for Tracks {}

#[derive(Clone)]
pub struct PlayClickedEvent {
    pub track: Arc<Track>,
}

#[derive(Clone)]
pub struct QueueClickedEvent {
    pub track: Arc<Track>,
}

#[derive(Clone)]
pub struct RightClickEvent {
    pub position: Point<Pixels>,
    pub items: Arc<Vec<ContextMenuItem>>,
}

#[derive(Clone)]
pub enum PlaybackEvent {
    TrackStarted(Arc<Track>),
    TrackEnded,
    Paused,
    Resumed,
}
impl PlaybackEvent {
    pub fn start(track: &Arc<Track>) -> Arc<PlaybackEvent> {
        Arc::new(PlaybackEvent::TrackStarted(Arc::clone(track)))
    }
}
impl gpui::EventEmitter<Arc<PlaybackEvent>> for Playback {}

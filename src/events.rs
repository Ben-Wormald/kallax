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
    RightClick(RightClickEvent),
}
impl UiEvent {
    pub fn play(track: &Arc<Track>) -> Arc<UiEvent> {
        Arc::new(UiEvent::PlayClicked(PlayClickedEvent { track: Arc::clone(track) }))
    }

    pub fn queue(track: &Arc<Track>) -> Arc<UiEvent> {
        Arc::new(UiEvent::QueueClicked(QueueClickedEvent { track: Arc::clone(track) }))
    }
}
impl gpui::EventEmitter<Arc<UiEvent>> for Tracks {}
impl gpui::EventEmitter<Arc<UiEvent>> for NowPlaying {}
impl gpui::EventEmitter<Arc<UiEvent>> for ContextMenu {}

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
impl gpui::EventEmitter<RightClickEvent> for Tracks {}

use std::sync::Arc;

use crate::*;

use context_menu::ContextMenuItem;

#[derive(Clone)]
pub enum Event {
    Play(PlayEvent),
    Queue(QueueEvent),
    Pause,
    Resume,
    Skip,
}
impl Event {
    pub fn play(track: &Arc<Track>) -> Arc<Event> {
        Arc::new(Event::Play(PlayEvent { track: Arc::clone(track) }))
    }

    pub fn queue(track: &Arc<Track>) -> Arc<Event> {
        Arc::new(Event::Queue(QueueEvent { track: Arc::clone(track) }))
    }
}
impl gpui::EventEmitter<Arc<Event>> for Tracks {}
impl gpui::EventEmitter<Arc<Event>> for NowPlaying {}
impl gpui::EventEmitter<Arc<Event>> for ContextMenu {}

#[derive(Clone)]
pub struct PlayEvent {
    pub track: Arc<Track>,
}

#[derive(Clone)]
pub struct QueueEvent {
    pub track: Arc<Track>,
}

pub struct RightClickEvent {
    pub position: Point<Pixels>,
    pub items: Arc<Vec<ContextMenuItem>>,
}
impl gpui::EventEmitter<RightClickEvent> for Tracks {}

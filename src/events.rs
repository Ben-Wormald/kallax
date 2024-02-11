use std::sync::Arc;

use crate::*;

use context_menu::ContextMenuItem;

pub enum Event {
    PlayEvent(PlayEvent),
    QueueEvent(QueueEvent),
}

impl gpui::EventEmitter<Arc<Event>> for ContextMenu {}

pub struct PlayEvent {
    pub track: Arc<Track>,
}
impl gpui::EventEmitter<PlayEvent> for Tracks {}
impl gpui::EventEmitter<PlayEvent> for ContextMenu {}

pub struct QueueEvent {
    pub track: Arc<Track>,
}
impl gpui::EventEmitter<QueueEvent> for ContextMenu {}

pub struct RightClickEvent {
    pub position: Point<Pixels>,
    pub items: Arc<Vec<ContextMenuItem>>,
}
impl gpui::EventEmitter<RightClickEvent> for Tracks {}

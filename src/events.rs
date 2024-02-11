use std::sync::Arc;

use crate::*;

use context_menu::ContextMenuItem;

#[derive(Clone)]
pub enum Event {
    Play(PlayEvent),
    Queue(QueueEvent),
}

impl gpui::EventEmitter<Arc<Event>> for ContextMenu {}

#[derive(Clone)]
pub struct PlayEvent {
    pub track: Arc<Track>,
}
impl gpui::EventEmitter<PlayEvent> for Tracks {}
impl gpui::EventEmitter<PlayEvent> for ContextMenu {}

#[derive(Clone)]
pub struct QueueEvent {
    pub track: Arc<Track>,
}
impl gpui::EventEmitter<QueueEvent> for ContextMenu {}

pub struct RightClickEvent {
    pub position: Point<Pixels>,
    pub items: Arc<Vec<ContextMenuItem>>,
}
impl gpui::EventEmitter<RightClickEvent> for Tracks {}

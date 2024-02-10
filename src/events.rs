use std::sync::Arc;

use crate::*;

pub struct PlayEvent {
    pub track: Arc<Track>,
}
impl  gpui::EventEmitter<PlayEvent> for Tracks {}

pub struct RightClickEvent {
    pub position: Point<Pixels>,
    pub items: Vec<String>,
}
impl  gpui::EventEmitter<RightClickEvent> for Tracks {}

use std::sync::Arc;

use crate::*;

pub struct PlayEvent {
    pub track: Arc<Track>,
}
impl  gpui::EventEmitter<PlayEvent> for Tracks {}

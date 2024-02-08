use crate::*;

pub struct PlayEvent {
    pub track: Track,
}
impl  gpui::EventEmitter<PlayEvent> for Tracks {}

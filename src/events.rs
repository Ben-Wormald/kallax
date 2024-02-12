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

impl gpui::EventEmitter<Arc<Event>> for ContextMenu {}
impl gpui::EventEmitter<Arc<Event>> for NowPlaying {}

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

// pub struct PauseEvent;
// impl gpui::EventEmitter<PauseEvent> for NowPlaying {}

// pub struct ResumeEvent;
// impl gpui::EventEmitter<ResumeEvent> for NowPlaying {}

// pub struct SkipEvent;
// impl gpui::EventEmitter<SkipEvent> for NowPlaying {}

pub struct RightClickEvent {
    pub position: Point<Pixels>,
    pub items: Arc<Vec<ContextMenuItem>>,
}
impl gpui::EventEmitter<RightClickEvent> for Tracks {}

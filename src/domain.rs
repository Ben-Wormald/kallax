use gpui::ImageData;
use std::sync::Arc;

#[derive(Clone)]
pub struct Track {
    pub path: String,
    pub name: String,
    pub artwork: Option<Arc<ImageData>>,
}

use gpui::ImageData;
use id3::Tag;
use image::{jpeg::JpegDecoder, DynamicImage};
use std::sync::Arc;

pub fn get_image(tags: &Tag) -> Option<Arc<ImageData>> {
    tags.pictures().next().and_then(|picture| {
        let decoder = JpegDecoder::new(picture.data.as_slice()).ok()?;
        let image = DynamicImage::from_decoder(decoder).ok()?;
        Some(Arc::new(ImageData::new(image.to_bgra8())))
    })
}

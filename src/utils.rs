use gpui::ImageData;
use id3::Tag;
use image::{jpeg::JpegDecoder, DynamicImage};
use std::sync::Arc;

pub fn get_image(tags: &Tag) -> Option<Arc<ImageData>> {
    tags.pictures().next().map(|picture| {
        let decoder = JpegDecoder::new(picture.data.as_slice()).unwrap();
        let image = DynamicImage::from_decoder(decoder).unwrap();
        Arc::new(ImageData::new(image.to_bgra8()))
    })
}

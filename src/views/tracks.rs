use gpui::*;
use id3::{Tag, TagLike};

use crate::*;

pub struct Tracks {
    tracks: Vec<Track>,
}

impl Tracks {
    pub fn new() -> Tracks {
        let dir = "/Users/ben/Music/Alvvays/Antisocialites";
        // let dir = "/Users/wormab01/Music/Skee Mask - Compro";

        let tracks = std::fs::read_dir(dir).unwrap()
            .filter_map(|entry| {
                let path = entry.unwrap().path();

                if path.extension().is_some_and(|extension| extension == "mp3") {
                    let path = path.to_str().unwrap().to_string();
                    let tags = Tag::read_from_path(&path).unwrap();
                    let name = tags.title().unwrap().to_string();
                    let artwork = utils::get_image(&tags);

                    Some(Track {
                        name,
                        path,
                        artwork,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<Track>>();

        Tracks { tracks }
    }
}

impl Render for Tracks {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .children(
                self.tracks.clone().into_iter().map(|track|
                    elements::track(track, cx)
                )
            )
    }
}

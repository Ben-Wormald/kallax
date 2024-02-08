use gpui::*;
use id3::{Tag, TagLike};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader, sync::Arc};

const COLOUR_BG: u32 = 0x333531;
const COLOUR_BORDER: u32 = 0x1f211f;
const COLOUR_TEXT: u32 = 0xf2f4f3;

actions!(musicplayer, [Quit]);

#[derive(Clone)]
struct Track {
    path: String,
    name: String,
}

struct MusicPlayer {
    _player: Model<Player>,
    tracks: View<Tracks>,
    now_playing: View<NowPlaying>,
}
impl MusicPlayer {
    fn new(cx: &mut ViewContext<MusicPlayer>) -> MusicPlayer {
        let player = cx.new_model(|_cx| Player::new());
        let tracks = cx.new_view(|_cx| Tracks::new());
        let now_playing = cx.new_view(|_cx| NowPlaying::new());

        cx.subscribe(&tracks, move |_subscriber, _emitter, event, cx| {
            Player::play(event.track.clone(), cx);
        }).detach();

        cx.update_view(&now_playing, |_, cx| {
            cx.subscribe(&tracks, move |subscriber, _emitter, event, _cx| {
                subscriber.track = Some(event.track.clone());
            }).detach();
        });

        MusicPlayer {
            _player: player,
            tracks,
            now_playing,
        }
    }
}
impl Render for MusicPlayer {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(COLOUR_BG))
            .size_full()
            .text_color(rgb(COLOUR_TEXT))
            .font("Work Sans")
            .child(self.tracks.clone())
            .child(self.now_playing.clone())
    }
}

struct Tracks {
    tracks: Vec<Track>,
}
impl Tracks {
    fn new() -> Tracks {
        // let path = "/Users/ben/Music/Alvvays/Antisocialites";
        let dir = "/Users/wormab01/Music/Skee Mask - Compro";

        let tracks = std::fs::read_dir(dir).unwrap()
            .filter_map(|entry| {
                let path = entry.unwrap().path();

                if path.extension().unwrap() == "mp3" {
                    let path = path.to_str().unwrap().to_string();
                    let name = Tag::read_from_path(&path).unwrap().title().unwrap().to_string();
                    Some(Track { name, path })
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
                    render_track(track, cx)
                )
            )
    }
}

struct NowPlaying {
    track: Option<Track>,
}
impl NowPlaying {
    fn new() -> NowPlaying {
        NowPlaying {
            track: None,
        }
    }
}
impl Render for NowPlaying {
    fn render(&mut self, _cx: &mut ViewContext<NowPlaying>) -> impl IntoElement {
        div()
            .child("Now playing:")
            .child(self.track.clone().map_or("-".to_string(), |track| track.name))
            .border()
            .border_color(rgb(COLOUR_BORDER))
            .size_full()
    }
}

fn render_track(track: Track, cx: &mut ViewContext<Tracks>) -> impl IntoElement {
    div()
        .id(ElementId::Name(track.name.clone().into()))
        .hover(|style| style.bg(rgb(COLOUR_BORDER)))
        .child(track.name.clone())
        .on_click(cx.listener(move |_this, _event, cx| {
            let track = track.clone();
            cx.emit(PlayEvent { track })
        }))
}

struct Player {
    sink: Arc<Sink>,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
}
impl Player {
    fn new() -> Player {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let sink = Arc::new(sink);

        Player {
            sink,
            _stream,
            _stream_handle: stream_handle,
        }
    }

    fn play(track: Track, cx: &mut AppContext) {
        let sink = cx.global::<Player>().sink.clone();
        cx.background_executor().spawn(async move {
            let file = BufReader::new(File::open(track.path).unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source);
            sink.sleep_until_end();
        }).detach();
    }
}

// #[derive(Debug)]
struct PlayEvent {
    track: Track,
}
impl EventEmitter<PlayEvent> for Tracks {}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        cx.set_global(Player::new());

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx: &mut ViewContext<MusicPlayer>| MusicPlayer::new(cx))
        });
    });
}

use std::{fs::File, io::BufReader, sync::Arc};
use gpui::*;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

const COLOUR_BG: u32 = 0x333531;
const COLOUR_BORDER: u32 = 0x1f211f;
const COLOUR_TEXT: u32 = 0xf2f4f3;

actions!(musicplayer, [Quit]);

struct MusicPlayer {
    is_playing: bool,
    player: Model<Player>,
    files: View<Files>,
}
impl MusicPlayer {
    fn new(cx: &mut ViewContext<MusicPlayer>) -> MusicPlayer {
        let player = cx.new_model(|_cx| Player::new());
        let files = cx.new_view(|_cx| Files::new());

        cx.subscribe(&files, move |_subscriber, _emitter, _event, cx| {
            Player::play(cx);
        }).detach();

        MusicPlayer {
            is_playing: false,
            player,
            files,
        }
    }
}
impl Render for MusicPlayer {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.files.clone()
    }
}

struct Files {
    names: Vec<String>,
}
impl Files {
    fn new() -> Files {
        // let path = "/Users/ben/Music/Alvvays/Antisocialites";
        let path = "/Users/wormab01/Music/Skee Mask - Compro";

        let names = std::fs::read_dir(path).unwrap()
            .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        Files { names }
    }
}
impl Render for Files {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text = self.names.clone().into_iter().map(|name| name);

        div()
            .flex()
            .bg(rgb(COLOUR_BG))
            .size_full()
            .text_color(rgb(COLOUR_TEXT))
            .font("Work Sans")
            .child(
                div()
                    .id("tracks")
                    .hover(|style| style.bg(rgb(COLOUR_BORDER)))
                    .children(text)
                    .border()
                    .border_color(rgb(COLOUR_BORDER))
                    .size_full()
                    .on_click(cx.listener(|_this, _event, cx| {
                        cx.emit(PlayEvent)
                    })),
            )
            .child(
                div().child("hi").border().border_color(rgb(COLOUR_BORDER)).size_full(),
            )
    }
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

    fn play(cx: &mut AppContext) {
        let sink = cx.global::<Player>().sink.clone();
        cx.background_executor().spawn(async move {
            let file = BufReader::new(File::open("/Users/wormab01/Music/Skee Mask - Compro/Skee Mask - ITLP04 - Compro - 06 Soundboy Ext..mp3").unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source);
            sink.sleep_until_end();
        }).detach();
    }
}

#[derive(Debug)]
struct PlayEvent;
impl EventEmitter<PlayEvent> for Files {}

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

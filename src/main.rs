use std::{fs::File, io::BufReader, thread};

use gpui::*;
use rodio::{Decoder, OutputStream, Sink};

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

        let player2 = player.clone();
        cx.subscribe(&files, move |_subscriber, _emitter, _event, cx| {

            Player::play(cx);
            // let player = player.clone();
            // let player = player2.clone();
            // cx.spawn(move |_this, mut cx| async move {
            //     player2.update(cx, |player, _cx| {
            //         player.play();
            //     });
            // }).detach();
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
                    .id("manage-members")
                    .children(text)
                    .border()
                    .border_color(rgb(COLOUR_BORDER))
                    .size_full()
                    .on_click(cx.listener(|_this, _event, cx| {
                        cx.emit(PlayEvent)
                    })),
            )
            .child(
                div().child("tracks").border().border_color(rgb(COLOUR_BORDER)).size_full(),
            )
    }
}

struct Player {
    sink: Sink,
}
impl Player {
    fn new() -> Player {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Player {
            sink,
        }
    }

    fn play(cx: &mut AppContext) {
        println!("play");
        cx.update_global::<Self, _>(|this, cx| {
            let file = BufReader::new(File::open("/Users/wormab01/Music/Skee Mask - Compro/Skee Mask - ITLP04 - Compro - 06 Soundboy Ext..mp3").unwrap());
            let source = Decoder::new(file).unwrap();
            this.sink.append(source);
            dbg!(this.sink.volume());
            dbg!(this.sink.is_paused());
            // self.sink.sleep_until_end();
            println!("play3");
        });
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

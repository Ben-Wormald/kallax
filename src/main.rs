use gpui::*;

const COLOUR_BG: u32 = 0x333531;
const COLOUR_BORDER: u32 = 0x1f211f;
const COLOUR_TEXT: u32 = 0xf2f4f3;

actions!(musicplayer, [Quit]);

struct MusicPlayer {
    player: Model<Player>,
    files: View<Files>,
}
impl MusicPlayer {
    fn new(cx: &mut ViewContext<MusicPlayer>) -> MusicPlayer {
        let player = cx.new_model(|_cx| Player {});
        let files = cx.new_view(|_cx| Files::new());

        player.update(cx, |_this, cx| {
            cx.subscribe(&files, |_subscriber, _emitter, event, _cx| {
                dbg!(event);
            }).detach();
        });

        MusicPlayer {
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
        let path = "/Users/ben/Music/Alvvays/Antisocialites";

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
                        dbg!("c");
                        cx.emit(PlayEvent)
                    })),
            )
            .child(
                div().child("tracks").border().border_color(rgb(COLOUR_BORDER)).size_full(),
            )
    }
}

struct Player;

#[derive(Debug)]
struct PlayEvent;
impl EventEmitter<PlayEvent> for Files {}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx: &mut ViewContext<MusicPlayer>| MusicPlayer::new(cx))
        });
    });
}

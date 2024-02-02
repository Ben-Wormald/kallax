use gpui::*;

const COLOUR_BG: u32 = 0x333531;
const COLOUR_BORDER: u32 = 0x1f211f;
const COLOUR_TEXT: u32 = 0xf2f4f3;

actions!(zed, [Quit]);

struct Files {
    names: Vec<String>,
}

struct Player;

#[derive(Debug)]
struct PlayEvent;
impl EventEmitter<PlayEvent> for HelloWorld {}

struct HelloWorld {
    text: Vec<String>,
}
impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text = self.text.clone().into_iter().map(|name| name);
        div()
            .flex()
            .bg(rgb(COLOUR_BG))
            .size_full()
            .text_color(rgb(COLOUR_TEXT))
            .font("Work Sans")
            .child(
                div().id("manage-members").children(text).border().border_color(rgb(COLOUR_BORDER)).size_full().on_click(cx.listener(|this, event, cx| {
                    cx.emit(PlayEvent)
                })),
            )
            .child(
                div().child("tracks").border().border_color(rgb(COLOUR_BORDER)).size_full(),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        let files: Model<Files> = cx.new_model(|_cx| Files { names: vec![] });

        files_init(cx, &files);

        let player: Model<Player> = cx.new_model(|cx| {
            cx.subscribe(&files, |subscriber, _emitter, event, _cx| {
                println!("{:?}", event);
            }).detach();

            Player {}
        });

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| HelloWorld {
                text: files.read(cx).names.clone(),
            })
        });
    });
}

fn files_init(cx: &mut AppContext, files_model: &Model<Files>) {
    let path = "/Users/wormab01/Music/Skee Mask - Compro/";

    let files = std::fs::read_dir(path).unwrap().map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string()).collect::<Vec<String>>();

    files_model.update(cx, |this, _cx| this.names = files);
}

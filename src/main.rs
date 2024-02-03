use gpui::*;

const COLOUR_BG: u32 = 0x333531;
const COLOUR_BORDER: u32 = 0x1f211f;
const COLOUR_TEXT: u32 = 0xf2f4f3;

actions!(musicplayer, [Quit]);

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

        let player = cx.new_model(|_cx| Player {});

        cx.observe_new_views(move |_: &mut Files, cx| {
            let files = cx.view().clone();

            player.update(cx, |_this, cx| {
                cx.subscribe(&files, |_subscriber, _emitter, event, _cx| {
                    dbg!(event);
                }).detach();
            });
        }).detach();

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx: &mut ViewContext<Files>| Files::new())
        });
    });
}

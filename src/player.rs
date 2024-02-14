use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader, sync::{atomic::AtomicUsize, Arc}, time::Duration};

use crate::*;

pub struct Player {
    sink: Arc<Sink>,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
}
impl Player {
    pub fn new() -> Player {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let sink = Arc::new(sink);

        Player {
            sink,
            _stream,
            _stream_handle: stream_handle,
        }
    }

    fn get_sink(cx: &mut gpui::AppContext) -> Arc<Sink> {
        Arc::clone(&cx.global::<Model<Player>>().read(cx).sink)
    }

    fn get_source(track: Arc<Track>) -> Decoder<BufReader<File>> {
        let file = BufReader::new(File::open(track.path.clone()).unwrap());
        Decoder::new(file).unwrap()
    }

    fn emit(cx: &mut gpui::AppContext, event: Arc<PlaybackEvent>) {
        let player = cx.global::<Model<Player>>().clone();
        player.update(cx, |_this, cx| {
            cx.emit(event);
        });
    }

    pub fn play(track: Arc<Track>, cx: &mut gpui::AppContext) {
        let sink = Self::get_sink(cx);
        let signal = Arc::new(AtomicUsize::new(1));

        {
            let track = track.clone();
            let signal = signal.clone();
            cx.background_executor().spawn(async move {
                let source = rodio::source::Done::new(Self::get_source(track), signal);
                sink.clear();
                sink.append(source);
                sink.play();
            }).detach();
        }

        cx.spawn(|mut cx| async move {
            loop {
                std::thread::sleep(Duration::from_secs(1));
                if signal.load(std::sync::atomic::Ordering::SeqCst) == 0 {
                    dbg!(&signal);
                    cx.update_global(|this: &mut Model<Player>, cx| {
                        this.update(cx, |_this, cx| {
                            dbg!("track ended!");
                            cx.emit(Arc::new(PlaybackEvent::TrackEnded));
                        })
                    }).ok();
                    break;
                }
            }
        }).detach();

        Self::emit(cx, PlaybackEvent::start(&track));
    }

    pub fn queue(track: Arc<Track>, cx: &mut gpui::AppContext) {
        let sink = Self::get_sink(cx);
        cx.background_executor().spawn(async move {
            let source = Self::get_source(track);
            sink.append(source);
        }).detach();
    }

    pub fn pause(cx: &mut gpui::AppContext) {
        let sink = Self::get_sink(cx);
        cx.background_executor().spawn(async move {
            sink.pause();
        }).detach();
    }

    pub fn resume(cx: &mut gpui::AppContext) {
        let sink = Self::get_sink(cx);
        cx.background_executor().spawn(async move {
            sink.play();
        }).detach();
    }

    pub fn skip(cx: &mut gpui::AppContext) {
        let sink = Self::get_sink(cx);
        cx.background_executor().spawn(async move {
            sink.skip_one();
        }).detach();
    }
}

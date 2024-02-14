use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader, sync::{atomic::AtomicUsize, Arc}, time::Duration};

use crate::*;

// https://github.com/aschey/stream-download-rs/tree/main/examples
// https://docs.rs/axum-streams/latest/axum_streams/index.html

struct Queue {
    tracks: Vec<QueueTrack>,
    current: Option<usize>,
}
impl Queue {
    fn new() -> Queue {
        Queue {
            tracks: Vec::new(),
            current: None,
        }
    }

    fn clear(&mut self) {
        self.tracks = Vec::new();
        self.current = None;
    }

    fn get_current(&self) -> Option<Arc<Track>> {
        self.current
            .and_then(|index| self.tracks.get(index))
            .map(|track| track.track.clone())
    }

    fn next(&mut self) {
        self.current = self.current.and_then(|index|
            if index + 1 < self.tracks.len() {
                Some(index + 1)
            } else {
                None
            }
        );
    }

    fn get_next(&mut self) -> Option<Arc<Track>> {
        self.next();
        self.get_current()
    }

    fn queue(&mut self, track: Arc<Track>) {
        self.tracks.push(QueueTrack::new(track));
    }
}

#[derive(Clone)]
struct QueueTrack {
    track: Arc<Track>,
    signal: Arc<AtomicUsize>,
}
impl QueueTrack {
    fn new(track: Arc<Track>) -> QueueTrack {
        QueueTrack {
            track,
            signal: Arc::new(AtomicUsize::new(1)),
        }
    }
}

pub struct Player {
    queue: Arc<Queue>,
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
            queue: Arc::new(Queue::new()),
            sink,
            _stream,
            _stream_handle: stream_handle,
        }
    }

    fn watch_signals(cx: &mut gpui::AppContext) {
        let queue = Self::get_queue(cx);

        cx.spawn(|mut cx| async move {
            loop {
                std::thread::sleep(Duration::from_secs(1));
                for mut track in queue.tracks.clone() {
                    if track.signal.load(std::sync::atomic::Ordering::SeqCst) == 0 {
                        cx.update_global(|this: &mut Model<Player>, cx| {
                            this.update(cx, |this, cx| {
                                this.queue.next();
                                cx.emit(Arc::new(PlaybackEvent::TrackEnded));
                            });
                        }).ok();
                        break;
                    }
                }
            }
        }).detach();
    }

    fn get_sink(cx: &mut gpui::AppContext) -> Arc<Sink> {
        Arc::clone(&cx.global::<Model<Player>>().read(cx).sink)
    }

    fn get_queue(cx: &mut gpui::AppContext) -> Arc<Queue> {
        Arc::clone(&cx.global::<Model<Player>>().read(cx).queue)
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

        {
            let track = QueueTrack::new(track.clone());
            cx.background_executor().spawn(async move {
                let source = rodio::source::Done::new(
                    Self::get_source(track.track),
                    track.signal,
                );
                sink.clear();
                sink.append(source);
                sink.play();
            }).detach();
        }

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

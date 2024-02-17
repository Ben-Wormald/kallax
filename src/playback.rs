use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader, sync::{atomic::AtomicUsize, Arc}, time::Duration};

use crate::*;

// https://github.com/aschey/stream-download-rs/tree/main/examples
// https://docs.rs/axum-streams/latest/axum_streams/index.html

pub struct Playback {
    queue: Arc<Queue>,
    player: Arc<Player>,
}
impl Playback {
    pub fn new(cx: &mut gpui::ModelContext<Playback>) -> Playback {
        let queue = Arc::new(Queue::new());

        let player = Player::new()
            .on_track_end({
                let queue = Arc::clone(&queue);
                move || {
                    queue.next();
                    Self::emit(cx, Arc::new(PlaybackEvent::TrackEnded));
                }
            });

        player.watch(cx);

        let player = Arc::new(player);

        Playback {
            queue,
            player,
        }
    }

    pub fn play(track: Arc<Track>, cx: &mut gpui::AppContext) {
        Self::get_player(cx).play(&track, cx);
        Self::emit(cx, PlaybackEvent::start(&track));
    }

    pub fn skip(cx: &mut gpui::AppContext) {
        Self::get_player(cx).skip(cx);
    }

    fn emit(cx: &mut gpui::AppContext, event: Arc<PlaybackEvent>) {
        let playback = cx.global::<Model<Playback>>().clone();
        playback.update(cx, |_this, cx| {
            cx.emit(event);
        });
    }

    fn get_player(cx: &mut gpui::AppContext) -> Arc<Player> {
        Arc::clone(&cx.global::<Model<Playback>>().read(cx).player)
    }

    fn get_queue(cx: &mut gpui::AppContext) -> Arc<Queue> {
        Arc::clone(&cx.global::<Model<Playback>>().read(cx).queue)
    }
}

struct Queue {
    tracks: Vec<Arc<Track>>,
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
            .map(|track| track.clone())
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

    fn queue(&mut self, track: &Arc<Track>) {
        self.tracks.push(Arc::clone(track));
    }
}

pub struct Player {
    sink: Arc<Sink>,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    track_end: Arc<Option<Box<dyn Fn() + Sync + Send>>>,
    queue_len: Arc<AtomicUsize>,
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
            track_end: Arc::new(None),
            queue_len: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn on_track_end(mut self, track_end: impl Fn() + Send + Sync + 'static) -> Self {
        self.track_end = Arc::new(Some(Box::new(track_end)));
        self
    }

    fn watch(&self, cx: &mut gpui::ModelContext<Playback>) {
        let queue_len = Arc::clone(&self.queue_len);
        let on_track_end = self.track_end.clone();

        cx.background_executor().spawn(async move {
            let mut prev_len = queue_len.load(std::sync::atomic::Ordering::SeqCst);

            loop {
                let current_len = queue_len.load(std::sync::atomic::Ordering::SeqCst);
                if current_len < prev_len {
                    if let Some(ref on_track_end) = *on_track_end {
                        on_track_end();
                    }
                }
                prev_len = current_len;
                std::thread::sleep(Duration::from_secs(1));
            }
        }).detach();
    }

    fn get_source(track: Arc<Track>) -> Decoder<BufReader<File>> {
        let file = BufReader::new(File::open(track.path.clone()).unwrap());
        Decoder::new(file).unwrap()
    }

    fn play(&self, track: &Arc<Track>, cx: &mut gpui::AppContext) {
            let track = QueueTrack::new(track);
            let sink = Arc::clone(&self.sink);
            let queue_len = Arc::clone(&self.queue_len);

            queue_len.store(1, std::sync::atomic::Ordering::SeqCst);

            cx.background_executor().spawn(async move {
                let source = rodio::source::Done::new(
                    Self::get_source(track.track),
                    queue_len,
                );
                sink.clear();
                sink.append(source);
                sink.play();
            }).detach();
    }

    fn queue(&self, track: Arc<Track>, cx: &mut gpui::AppContext) {
        let sink = Arc::clone(&self.sink);
        cx.background_executor().spawn(async move {
            let source = Self::get_source(track);
            sink.append(source);
        }).detach();
    }

    fn pause(&self, cx: &mut gpui::AppContext) {
        let sink = Arc::clone(&self.sink);
        cx.background_executor().spawn(async move {
            sink.pause();
        }).detach();
    }

    fn resume(&self, cx: &mut gpui::AppContext) {
        let sink = Arc::clone(&self.sink);
        cx.background_executor().spawn(async move {
            sink.play();
        }).detach();
    }

    fn skip(&self, cx: &mut gpui::AppContext) {
        let sink = Arc::clone(&self.sink);
        cx.background_executor().spawn(async move {
            sink.skip_one();
        }).detach();
    }
}

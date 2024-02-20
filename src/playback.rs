use gpui::{AppContext, ModelContext};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{
    fs::File,
    io::BufReader,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
    time::Duration,
};

use crate::*;

// https://github.com/aschey/stream-download-rs/tree/main/examples
// https://docs.rs/axum-streams/latest/axum_streams/index.html

const POLL_DURATION: Duration = Duration::from_millis(100);

pub struct Playback {
    pub queue: Queue,
    player: Player,
}
impl Playback {
    pub fn new(cx: &mut ModelContext<Playback>) -> Playback {
        let queue = Queue::new();

        let player = Player::new();
        player.watch(cx);

        Playback {
            queue,
            player,
        }
    }

    pub fn play(&mut self, track: Arc<Track>, cx: &mut AppContext) {
        self.player.play(&track, cx);
        self.queue.play(&track);
    }

    pub fn add_to_queue(&mut self, track: Arc<Track>, cx: &mut AppContext) {
        self.player.add_to_queue(&track, cx);
        self.queue.add_to_queue(&track);
    }

    pub fn pause(&mut self, cx: &mut AppContext) {
        self.player.pause(cx);
    }

    pub fn resume(&mut self, cx: &mut AppContext) {
        self.player.resume(cx);
    }

    pub fn skip(&mut self, cx: &mut AppContext) {
        self.player.skip(cx);
    }

    fn on_track_end(&mut self, cx: &mut ModelContext<Playback>) {
        self.queue.next();
        cx.notify();
    }
}

pub struct Queue {
    pub tracks: Vec<Arc<Track>>,
    pub current: Option<usize>,
}
impl Queue {
    fn new() -> Queue {
        Queue {
            tracks: Vec::new(),
            current: None,
        }
    }

    fn play(&mut self, track: &Arc<Track>) {
        self.tracks = vec![Arc::clone(track)];
        self.current = Some(0);
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

    fn add_to_queue(&mut self, track: &Arc<Track>) {
        self.tracks.push(Arc::clone(track));
    }
}

pub struct Player {
    sink: Arc<Sink>,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
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
            queue_len: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn watch(&self, cx: &mut ModelContext<Playback>) {
        let queue_len = Arc::clone(&self.queue_len);

        cx.spawn(|this, mut cx| async move {
            let mut prev_len = queue_len.load(SeqCst);

            loop {
                let current_len = queue_len.load(SeqCst);

                if current_len != prev_len {
                    println!("queue_len {prev_len} -> {current_len}");
                }

                if current_len < prev_len {
                    this.update(&mut cx, |playback, cx| {
                        playback.on_track_end(cx);
                    }).ok();
                }
                prev_len = current_len;
                cx.background_executor().timer(POLL_DURATION).await;
            }
        }).detach();
    }

    fn get_source(track: &Arc<Track>) -> Decoder<BufReader<File>> {
        let file = BufReader::new(File::open(track.path.clone()).unwrap());
        Decoder::new(file).unwrap()
    }

    fn play(&self, track: &Arc<Track>, cx: &mut AppContext) {
        let track = Arc::clone(track);
        let sink = Arc::clone(&self.sink);
        let queue_len = Arc::clone(&self.queue_len);

        queue_len.store(1, SeqCst);

        cx.background_executor().spawn(async move {
            let source = rodio::source::Done::new(
                Self::get_source(&track),
                queue_len,
            );
            sink.clear();
            sink.append(source);
            sink.play();
        }).detach();
    }

    fn add_to_queue(&self, track: &Arc<Track>, cx: &mut AppContext) {
        let track = Arc::clone(track);
        let sink = Arc::clone(&self.sink);
        let queue_len = Arc::clone(&self.queue_len);

        queue_len.fetch_add(1, SeqCst);

        cx.background_executor().spawn(async move {
            let source = rodio::source::Done::new(
                Self::get_source(&track),
                queue_len,
            );
            sink.append(source);
        }).detach();
    }

    fn pause(&self, cx: &mut AppContext) {
        let sink = Arc::clone(&self.sink);
        cx.background_executor().spawn(async move {
            sink.pause();
        }).detach();
    }

    fn resume(&self, cx: &mut AppContext) {
        let sink = Arc::clone(&self.sink);
        cx.background_executor().spawn(async move {
            sink.play();
        }).detach();
    }

    fn skip(&self, cx: &mut AppContext) {
        let sink = Arc::clone(&self.sink);
        let queue_len = Arc::clone(&self.queue_len);

        queue_len.fetch_update(SeqCst, SeqCst, |len| Some(len - 1)).ok();

        cx.background_executor().spawn(async move {
            sink.skip_one();
        }).detach();
    }
}

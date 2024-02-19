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

pub struct Playback {
    queue: Queue,
    player: Arc<Player>,
}
impl Playback {
    pub fn new(cx: &mut gpui::ModelContext<Playback>) -> Playback {
        let queue = Queue::new();

        let player = Player::new();
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

    pub fn queue(track: Arc<Track>, cx: &mut gpui::AppContext) {
        Self::get_player(cx).queue(&track, cx);
    }

    pub fn pause(cx: &mut gpui::AppContext) {
        Self::get_player(cx).pause(cx);
    }

    pub fn resume(cx: &mut gpui::AppContext) {
        Self::get_player(cx).resume(cx);
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

    // fn get_queue(cx: &mut gpui::AppContext) -> Arc<Queue> {
    //     Arc::clone(&cx.global::<Model<Playback>>().read(cx).queue)
    // }

    fn next(&mut self, cx: &mut gpui::ModelContext<Playback>) {
        self.queue.next();
        Self::emit(cx, Arc::new(PlaybackEvent::TrackEnded));
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

    fn watch(&self, cx: &mut gpui::ModelContext<Playback>) {
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
                        playback.next(cx);
                        // cx.emit(Arc::new(PlaybackEvent::TrackEnded))
                    }).ok();
                }
                prev_len = current_len;
                cx.background_executor().timer(Duration::from_millis(2000)).await;
            }
        }).detach();
    }

    fn get_source(track: &Arc<Track>) -> Decoder<BufReader<File>> {
        let file = BufReader::new(File::open(track.path.clone()).unwrap());
        Decoder::new(file).unwrap()
    }

    fn play(&self, track: &Arc<Track>, cx: &mut gpui::AppContext) {
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

    fn queue(&self, track: &Arc<Track>, cx: &mut gpui::AppContext) {
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
        let queue_len = Arc::clone(&self.queue_len);

        queue_len.fetch_update(SeqCst, SeqCst, |len| Some(len - 1)).ok();

        cx.background_executor().spawn(async move {
            sink.skip_one();
        }).detach();
    }
}

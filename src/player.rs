use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader, sync::Arc};

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

    pub fn play(track: Arc<Track>, cx: &mut gpui::AppContext) {
        let sink = Arc::clone(&cx.global::<Player>().sink);
        cx.background_executor().spawn(async move {
            let file = BufReader::new(File::open(track.path.clone()).unwrap());
            let source = Decoder::new(file).unwrap();
            sink.clear();
            sink.append(source);
            sink.play();
        }).detach();
    }

    pub fn queue(track: Arc<Track>, cx: &mut gpui::AppContext) {
        let sink = Arc::clone(&cx.global::<Player>().sink);
        cx.background_executor().spawn(async move {
            let file = BufReader::new(File::open(track.path.clone()).unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source);
        }).detach();
    }

    pub fn pause(cx: &mut gpui::AppContext) {
        let sink = Arc::clone(&cx.global::<Player>().sink);
        cx.background_executor().spawn(async move {
            sink.pause();
        }).detach();
    }

    pub fn resume(cx: &mut gpui::AppContext) {
        let sink = Arc::clone(&cx.global::<Player>().sink);
        cx.background_executor().spawn(async move {
            sink.play();
        }).detach();
    }

    pub fn skip(cx: &mut gpui::AppContext) {
        let sink = Arc::clone(&cx.global::<Player>().sink);
        cx.background_executor().spawn(async move {
            sink.skip_one();
        }).detach();
    }
}

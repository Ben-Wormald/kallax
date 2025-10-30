use gpui::{Context, Entity};
use objc2::{rc::Retained, runtime::{AnyObject, ProtocolObject}};
use objc2_avf_audio::{AVAudioSession, AVAudioSessionCategoryPlayback};
use objc2_foundation::{NSDictionary, NSMutableDictionary, NSNumber, NSObject, NSString};
use objc2_media_player::{MPMediaItemPropertyAlbumArtist, MPMediaItemPropertyAlbumTitle, MPMediaItemPropertyArtist, MPMediaItemPropertyMediaType, MPMediaItemPropertyPlaybackDuration, MPMediaItemPropertyTitle, MPNowPlayingInfoCenter, MPNowPlayingInfoMediaType, MPNowPlayingInfoPropertyDefaultPlaybackRate, MPNowPlayingInfoPropertyElapsedPlaybackTime, MPNowPlayingInfoPropertyPlaybackRate, MPNowPlayingPlaybackState};

use crate::{events::PlaybackEvent, models::Playback};

type Mcx<'a> = Context<'a, MacOS>;

pub struct MacOS {
}

impl MacOS {
    pub fn new(cx: &mut Mcx, playback: &Entity<Playback>) -> MacOS {
        cx.subscribe(playback, |this, _emitter, event, cx| {
            match (**event).clone() {
                PlaybackEvent::TrackStarted(track) => this.play(),
                PlaybackEvent::Paused => (),
                PlaybackEvent::Resumed => (),
                PlaybackEvent::TrackEnded => (),
            }
        }).detach();

        MacOS {
        }
    }

    fn play(&mut self) {
        unsafe {
            let session = AVAudioSession::sharedInstance();
            session.setCategory_error(AVAudioSessionCategoryPlayback.unwrap()).expect("failed to set category");
            session.setActive_error(true).expect("failed to set active");

            let info_center = MPNowPlayingInfoCenter::defaultCenter();

            let now_playing_info: Retained<NSDictionary<NSString, AnyObject>> = NSDictionary::from_slices(&[
                MPMediaItemPropertyMediaType,
                MPMediaItemPropertyTitle,
                MPMediaItemPropertyAlbumTitle,
                MPMediaItemPropertyArtist,
                MPMediaItemPropertyPlaybackDuration,
                MPNowPlayingInfoPropertyElapsedPlaybackTime,
                MPNowPlayingInfoPropertyPlaybackRate,
                MPNowPlayingInfoPropertyDefaultPlaybackRate,
            ], &[
                &NSNumber::new_usize(MPNowPlayingInfoMediaType::Audio.0),
                &NSString::from_str("test track"),
                &NSString::from_str("test album"),
                &NSString::from_str("test artist"),
                &NSNumber::new_f64(60.0),
                &NSNumber::new_f64(20.0),
                &NSNumber::new_f64(1.0),
                &NSNumber::new_f64(1.0),
            ]);

            info_center.setNowPlayingInfo(None);
            info_center.setNowPlayingInfo(Some(&*now_playing_info));
            info_center.setPlaybackState(MPNowPlayingPlaybackState::Playing);
        }
    }
}

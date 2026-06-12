use std::sync::Arc;

use librespot::core::session::Session;
use librespot::playback::config::AudioFormat;
use librespot::playback::config::PlayerConfig;
use librespot::playback::player::{Player, PlayerEvent};
use librespot::playback::audio_backend;
use librespot::playback::mixer::NoOpVolume;

use crate::track::TrackMeta;

pub fn create_player(session: Session) -> Arc<Player> {
    let sink_builder = audio_backend::find(None).expect("Audio backends to listen not found");
    let audio_format: AudioFormat = AudioFormat::default();
    Player::new(
            PlayerConfig::default(),
            session,
            Box::new(NoOpVolume),               
            move || sink_builder(None, audio_format),
    )
}


pub async fn record_track(session: Session, track: TrackMeta) {
    let player = create_player(session);
    player.load(track.id, true, 0);
    let mut event_rc = player.get_player_event_channel();

    tokio::spawn(async move {
        while let Some(event) = event_rc.recv().await {
            match event {
                PlayerEvent::Playing { .. } => {

                },
                PlayerEvent::EndOfTrack { .. } => {

                },
                _ => {}
            }
        }
    });

}

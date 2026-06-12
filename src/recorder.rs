use std::sync::Arc;
use std::fs;
use std::io::Write;
use std::error::Error;

use ffmpeg_sidecar::command::FfmpegCommand;
use ffmpeg_sidecar::child::FfmpegChild;
use id3::Timestamp;
use librespot::core::session::Session;
use librespot::playback::config::AudioFormat;
use librespot::playback::config::PlayerConfig;
use librespot::playback::player::{Player, PlayerEvent};
use librespot::playback::audio_backend;
use librespot::playback::mixer::NoOpVolume;
use ffmpeg_sidecar::download::auto_download;
use id3::{Tag, TagLike};

use crate::track::TrackMeta;


struct AudioRecorder {
    ffmpeg_process: Option<FfmpegChild>,
    output_path: String,
    monitor: String,
    track: TrackMeta,
}

impl AudioRecorder {
    fn new(output_path: String, monitor: String, track: TrackMeta) -> Self {
        auto_download().unwrap();
        Self { 
            ffmpeg_process: None, 
            output_path: output_path, 
            monitor: monitor, 
            track: track 
        }
    }

    fn set_track_tags(&mut self) {
        let mut tag: Tag = Tag::read_from_path(self.output_path.clone()).unwrap_or_else(|_| Tag::new());
        let track = self.track.clone();
        tag.set_title(track.name);
        tag.set_album(track.album_name);
        tag.set_track(track.number as u32);
        tag.set_year(track.date.year());
        tag.set_date_released(Timestamp { 
                year: track.date.year(), 
                month: Some(u8::from(track.date.month())), 
                day: Some(track.date.day()), 
                hour: Some(track.date.hour()), 
                minute: Some(track.date.minute()), 
                second: Some(track.date.second()) 
            });
        tag.set_artist(track.artists[0].as_str());
        for artist in track.artists.iter().skip(1) {
            tag.set_text("TPE1", artist);
        }
        tag.set_album_artist(track.album_artists[0].as_str());
        for album_artist in track.album_artists.iter().skip(1) {
            tag.set_text("TPE2", album_artist);
        }  

        tag.write_to_path(self.output_path.clone(), id3::Version::Id3v24).unwrap();
    }

    fn start_recording(&mut self) {
        let child = FfmpegCommand::new()
            .args(&[
                "-f", "pulse",
                "-i", self.monitor.as_str(),
                "-c:a", "libmp3lame",
                "-b:a", "192k",
                &self.output_path,
            ]).spawn().unwrap();
        self.ffmpeg_process = Some(child);
    }

    fn stop_recording(&mut self) {
        if let Some(mut child) = self.ffmpeg_process.take() {
    
            if let Some(mut stdin) = child.take_stdin() {
                stdin.write_all(b"q").expect("Cannot send a quit signal");
                stdin.flush().expect("Cannot drop stdin buffer");
        }
    
        child.wait().expect("FFmpeg complete uncorrectly");
    }
    self.set_track_tags();
}}




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


pub async fn record_track(session: Session, track: TrackMeta, monitor: String) -> bool {
    let player = create_player(session);
    player.load(track.id.clone(), true, 0);
    let output_dir = "./data/";
    fs::create_dir_all(output_dir).expect("Cannot create data folder");
    let output_path = "./data/".to_string() + track.name.as_str() + ".mp3";

    println!("OUTPUT_PATH: {}", output_path);

    let mut recorder = AudioRecorder::new(output_path, monitor, track);

    let mut event_rc = player.get_player_event_channel();


    let handle = tokio::spawn(async move {
        while let Some(event) = event_rc.recv().await {
            match event {
                PlayerEvent::Playing { .. } => {
                    println!("START RECORDING..");
                    recorder.start_recording();
                }
                PlayerEvent::EndOfTrack { .. } => {
                    println!("STOP RECORDING..");
                    recorder.stop_recording();
                    return true;  
                }
                _ => continue,
            }
        }
        false  
    });
    handle.await.unwrap_or(false)
}

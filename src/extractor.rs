use librespot::core::{SessionConfig, session::Session};


use crate::config::Config;

pub async fn extract_playlist(conifg: Config, playlist_id: String) {
    let session: Session = Session::new(SessionConfig::default(), None);

}

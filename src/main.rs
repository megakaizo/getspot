mod extractor;
mod config;
mod recorder;
mod track;

use std::error::Error;

use crate::config::Config;
use crate::extractor::extract_playlist;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let monitor = "virtual_sink.monitor";
    let playlist_uri = "spotify:playlist:4ZlZah6Ks8E5f5GDOJns7a";
    let track_number: u32 = 60;
    let config: Config = Config::load_config()?;

    extract_playlist(config, playlist_uri.to_string(), monitor.to_string(), track_number).await?;

    Ok(())
}

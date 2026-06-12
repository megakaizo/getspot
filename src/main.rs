mod extractor;
mod config;

use std::error::Error;

use crate::config::Config;
use crate::extractor::extract_playlist;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let playlist_uri = "spotify:playlist:4ZlZah6Ks8E5f5GDOJns7a";

    let config: Config = Config::load_config()?;

    extract_playlist(config, playlist_uri.to_string()).await?;

    Ok(())
}

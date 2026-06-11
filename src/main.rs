mod extractor;
mod config;

use std::env;
use std::error::Error;
use dotenvy::dotenv;

use crate::config::Config;
use crate::extractor::extract_playlist;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let playlist_id = "4ZlZah6Ks8E5f5GDOJns7a";
    
    let config: Config = Config::load_config()?;

    extract_playlist(config, playlist_id.to_string()).await?;

    Ok(())
}

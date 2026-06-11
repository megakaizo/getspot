mod extractor;
mod config;

use std::env;
use dotenvy::dotenv;

use crate::config::Config;
use crate::extractor::extract_playlist;


#[tokio::main]
async fn main() {
    let playlist_id = "4ZlZah6Ks8E5f5GDOJns7a";
    
    let config: Config = Config::load_config()?;


    extract_playlist(conifg, playlist_id.to_string()).await?;
}

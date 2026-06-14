mod extractor;
mod recorder;
mod track;
mod cli;

use std::error::Error;

use clap::Parser;

use crate::cli::Cli;
use crate::extractor::extract_playlist;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {

    }    
    extract_playlist(playlist_uri.to_string(), monitor.to_string(), track_number).await?;

    Ok(())
}

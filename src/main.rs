mod extractor;
mod recorder;
mod track;
mod cli;

use std::error::Error;

use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::extractor::{
    extract_playlist, extract_album, extract_track
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Track(args) => {
            for id in args.ids.iter() {
                extract_track(id.clone(), args.audio_sink.clone(), args.save_to.clone()).await?; 
            }
        },
        Commands::Playlist(args) => {
            for id in args.ids.iter() {
                extract_playlist(id.clone(), args.audio_sink.clone(), args.start_number, args.save_to.clone()).await?; 
            }
        },
        Commands::Album(args) => {
            for id in args.ids.iter() {
                extract_album(id.clone(), args.audio_sink.clone(), args.save_to.clone()).await?; 
            }
        }
    }    

    Ok(())
}

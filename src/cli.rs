use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Args)]
pub struct ItemArgs {
    #[arg(num_args = 1..)]
    pub ids: Vec<String>,
    pub audio_sink: String,
    pub save_to: String,
}

#[derive(Args)]
pub struct ItemsArgs {
    #[arg(num_args = 1..)]
    pub ids: Vec<String>,
    pub audio_sink: String,
    pub start_number: u16, 
    pub save_to: String,
}


#[derive(Subcommand)]
pub enum Commands {

    Track(ItemArgs),    

    Playlist(ItemsArgs),

    Album(ItemsArgs),
}



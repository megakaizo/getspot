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
    #[arg(short = 's', long)]
    pub audio_sink: String,
    #[arg(short = 'o', long)]
    pub save_to: Option<String>,
}

#[derive(Args)]
pub struct ItemsArgs {
    #[arg(num_args = 1..)]
    pub ids: Vec<String>,
    #[arg(short = 's', long)]
    pub audio_sink: String,
    #[arg(short = 'n', long)]
    pub start_number: u16, 
    #[arg(short = 'o', long)]
    pub save_to: Option<String>,
}


#[derive(Subcommand)]
pub enum Commands {

    Track(ItemArgs),    

    Playlist(ItemsArgs),

    Album(ItemArgs),
}



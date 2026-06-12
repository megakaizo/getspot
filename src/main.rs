mod extractor;
mod config;
mod recorder;
mod track;

use std::error::Error;

use crate::config::Config;
use crate::extractor::extract_playlist;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let monitor = "alsa_output.usb-Generic_USB_Audio_Device_20230726905926-00.analog-stereo.monitor";
    let playlist_uri = "spotify:playlist:4ZlZah6Ks8E5f5GDOJns7a";

    let config: Config = Config::load_config()?;

    extract_playlist(config, playlist_uri.to_string(), monitor.to_string()).await?;

    Ok(())
}

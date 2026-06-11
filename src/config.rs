use std::error::Error;
use std::env;

use dotenvy::dotenv;

pub const SCOPES: &[&str] = &[
    "app-remote-control",
    "playlist-modify",
    "playlist-modify-private",
    "playlist-modify-public",
    "playlist-read",
    "playlist-read-collaborative",
    "playlist-read-private",
    "streaming",
    "ugc-image-upload",
    "user-follow-modify",
    "user-follow-read",
    "user-library-modify",
    "user-library-read",
    "user-modify",
    "user-modify-playback-state",
    "user-modify-private",
    "user-personalized",
    "user-read-birthdate",
    "user-read-currently-playing",
    "user-read-email",
    "user-read-play-history",
    "user-read-playback-position",
    "user-read-playback-state",
    "user-read-private",
    "user-read-recently-played",
    "user-top-read",
];

pub struct Config {
    pub web_access_token: String,
    pub client_id: String,
    pub redirect_uri: String,
}


impl Config {
    pub fn load_config() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let web_access_token = env::var("WEB_ACCESS_TOKEN").expect("WEB ACCESS TOKEN NOT SET");
        let client_id = env::var("CLIENT_ID").expect("CLIENT_ID NOT SET");
        let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI NOT SET");
        Ok(Config { web_access_token, client_id, redirect_uri }) 
    }
}


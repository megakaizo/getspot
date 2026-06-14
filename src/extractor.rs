use std::error::Error;

use librespot::core::{SpotifyUri};
use librespot::metadata::Metadata;
use librespot::oauth::OAuthClientBuilder;
use librespot::oauth::OAuthClient;
use librespot::core::{SessionConfig, authentication::Credentials, session::Session};
use librespot::oauth::OAuthToken;
use librespot::metadata::{Playlist, Track};

use crate::recorder::record_track;
use crate::track::TrackMeta;


const REDIRECT_URI: &str = "http://127.0.0.1:5588/login";
const CLIENT_ID: &str =  "65b708073fc0480ea92a077233ca87bd";
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



async fn get_access_token() -> OAuthToken {
    let scopes: Vec<&str> = SCOPES.to_vec();
    let builder: OAuthClientBuilder = OAuthClientBuilder::new(CLIENT_ID, REDIRECT_URI, scopes);        
    let client: OAuthClient = builder.build().unwrap();
    return client.get_access_token().unwrap()
}


pub async fn extract_track(track_id: &SpotifyUri, session: &Session) -> Result<TrackMeta, Box<dyn Error>> {
    let track: Track = Track::get(session, track_id).await.unwrap();

    let track_name = track.name;
    let album_name = track.album.name;
    let track_number = track.number;
    let artists_names: Vec<String> = track.artists.iter().map(|artist| artist.name.clone()).collect();
    let album_artists_names: Vec<String> = track.album.artists.iter().map(|artist| artist.name.clone()).collect();
    let date = track.album.date;
    let image_url = format!("https://i.scdn.co/image/{}", track.album.covers.first().unwrap().id);    
    
    println!("Track: {}", track_name);
    println!("Track #: {}", track_number);
    for artist_name in artists_names.clone() { 
        println!("Artists: {}", artist_name)
    }
    println!("Album: {}", album_name);
    println!("Date: {}", date.to_string());
    println!("Image URL: {}", image_url);
    Ok(
        TrackMeta{
            id: track.id, 
            name: track_name, 
            album_name: album_name, 
            number: track_number, 
            artists: artists_names, 
            album_artists: album_artists_names, 
            date: date, 
            image_url: image_url
        }
    )    
}

pub async fn extract_playlist(playlist_uri: String, monitor: String, track_number: u32) -> Result<(), Box<dyn Error>> {
    println!("start extracting playlist...");
    let token = get_access_token().await;

    let session: Session = Session::new(SessionConfig::default(), None);
    let credentials: Credentials = Credentials::with_access_token(token.access_token);
    session.connect(credentials, true).await?;
    println!("successfully connected to spotify session...");
    
    let playlist_id = SpotifyUri::from_uri(playlist_uri.as_str()).unwrap();
    let playlist: Playlist = Playlist::get(&session, &playlist_id).await?;
    
    for i in (track_number - 1) as usize..playlist.contents.items.len() {
        let track_item = &playlist.contents.items[i];
        let track_id = &track_item.id;
        let track_meta = extract_track(track_id, &session).await?;
        let record_status = record_track(session.clone(), track_meta, monitor.clone()).await;
        println!("Record status: {}", record_status);
    } 

    Ok(())

}

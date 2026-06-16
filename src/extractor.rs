use std::error::Error;

use librespot::core::{SpotifyUri};
use librespot::metadata::{Album, Metadata};
use librespot::oauth::OAuthClientBuilder;
use librespot::oauth::OAuthClient;
use librespot::core::{SessionConfig, authentication::Credentials, session::Session};
use librespot::oauth::OAuthToken;
use librespot::metadata::{Playlist, Track};

use crate::recorder::record_track;
use crate::track::TrackMeta;

// default librespot auth params
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


async fn get_connected_session(token: OAuthToken) -> Session {
    let session: Session = Session::new(SessionConfig::default(), None);
    let credentials: Credentials = Credentials::with_access_token(token.access_token);
    session.connect(credentials, true).await.unwrap();
    session
}

fn show_track_meta_info(track: &TrackMeta) {
    println!("Track: {}", track.name);
    println!("Track #: {}", track.number);
    for artist_name in track.artists.clone() { 
        println!("Artists: {}", artist_name)
    }
    println!("Album: {}", track.album_name);
    println!("Date: {}", track.date.to_string());
    println!("Image URL: {}", track.image_url);

}


pub async fn collect_track_meta(track_id: &SpotifyUri, session: &Session) -> Result<TrackMeta, Box<dyn Error>> {
    let track: Track = Track::get(session, track_id).await.unwrap();

    let track_name = track.name;
    let album_name = track.album.name;
    let track_number = track.number;
    let artists_names: Vec<String> = track.artists.iter().map(|artist| artist.name.clone()).collect();
    let album_artists_names: Vec<String> = track.album.artists.iter().map(|artist| artist.name.clone()).collect();
    let date = track.album.date;
    let image_url = format!("https://i.scdn.co/image/{}", track.album.covers.first().unwrap().id);    
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


pub async fn extract_playlist(
    playlist_id: String, monitor: String, track_number: u16, save_path: Option<String>
) -> Result<(), Box<dyn Error>> {
    println!("start extracting playlist...");
    let raw_uri = format!("spotify:playlist:{}", playlist_id);

    let token = get_access_token().await;
    let session = get_connected_session(token).await;
    println!("successfully connected to spotify session...");
    
    let playlist_uri = SpotifyUri::from_uri(raw_uri.as_str()).unwrap();
    let playlist: Playlist = Playlist::get(&session, &playlist_uri).await?;
    
    for i in (track_number - 1) as usize..playlist.contents.items.len() {
        let track_item = &playlist.contents.items[i];
        let track_id = &track_item.id;
        let track = collect_track_meta(track_id, &session).await?;
        show_track_meta_info(&track);

        let record_status = record_track(session.clone(), track, monitor.clone(), save_path.clone()).await;
        println!("Record status: {}", record_status);
    } 

    Ok(())

}


pub async fn extract_track(
    track_id: String, monitor: String, save_path: Option<String>
) -> Result<(), Box<dyn Error>> {
    println!("start extracting track...");
    let raw_uri = format!("spotify:track:{}", track_id);
    
    let token = get_access_token().await;
    let session = get_connected_session(token).await;   
    println!("successfully connected to spotify session...");

    let track_uri = SpotifyUri::from_uri(raw_uri.as_str()).unwrap();
    let track = collect_track_meta(&track_uri, &session).await?;
    show_track_meta_info(&track);

    let record_status = record_track(session.clone(), track, monitor, save_path).await;
    println!("Record status: {}", record_status);
    Ok(())
}


pub async fn extract_album(
    album_id: String, monitor: String, save_path: Option<String>, 
) -> Result<(), Box<dyn Error>> {
    println!("start extracting album...");
    let raw_uri = format!("spotify:album:{}", album_id);
    
    let token = get_access_token().await;
    let session = get_connected_session(token).await;   
    println!("successfully connected to spotify session...");
    
    let album_uri = SpotifyUri::from_uri(raw_uri.as_str()).unwrap();
    let album: Album = Album::get(&session, &album_uri).await?;

    for i in 0..album.discs.len() {
        let disc = &album.discs[i];
        for j in 0..disc.tracks.len() {
            let track_id = &disc.tracks[j];
            let track = collect_track_meta(track_id, &session).await?;
            show_track_meta_info(&track);

            let record_status = record_track(session.clone(), track, monitor.clone(), save_path.clone()).await;
            println!("Record status: {}", record_status);

        }
    }
    Ok(())

}

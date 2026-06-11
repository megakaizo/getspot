use std::error::Error;

use librespot::oauth::OAuthClientBuilder;
use librespot::oauth::OAuthClient;
use librespot::core::{SessionConfig, authentication::Credentials, session::Session};
use crate::config::SCOPES;

use crate::config::Config;

pub async fn extract_playlist(config: Config, playlist_id: String) -> Result<(), Box<dyn Error>> {
    println!("start extracting playlist...");
    println!("{}, {}", config.client_id, config.redirect_uri);
    let scopes: Vec<&str> = SCOPES.to_vec();
    let builder: OAuthClientBuilder = OAuthClientBuilder::new(config.client_id.trim(), config.redirect_uri.trim(), scopes);        
    let client: OAuthClient = builder.build().unwrap();
    let token = client.get_access_token().unwrap();

    let session: Session = Session::new(SessionConfig::default(), None);
    let credentials: Credentials = Credentials::with_access_token(token.access_token);
    session.connect(credentials, true).await?;

    println!("successfully connected to spotify session...");

    Ok(())

}

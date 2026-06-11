use std::error::Error;
use std::env;

use dotenvy::dotenv;


pub struct Config {
    web_access_token: String,
    auth_token: String,
}


impl Config {
    pub fn load_config() -> Result<Self, Box<dyn Error>> {
        let web_acess_token = env::var("WEB_ACESS_TOKEN").expect("WEB ACESS TOKEN NOT SET");
        let auth_token = env::var("AUTH_DATA").expect("AUTH DATA NOT SET");
        Ok(Config{web_access_token, auth_token})
    }
}


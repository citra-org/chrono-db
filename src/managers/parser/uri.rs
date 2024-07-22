use url::Url;
use std::error::Error;
use std::io::{ErrorKind, Error as IoError};


pub struct ConnectionInfo {
    pub keeper: String,
    pub secret: String,
    pub host: String,
    pub port: u16,
    pub chrono: String,
}

pub fn parse_itlg_uri(url: &str) -> Result<ConnectionInfo, Box<dyn Error + Send + Sync>> {
    let parsed_url = Url::parse(url)?;

    if parsed_url.scheme() != "itlg" {
        return Err(Box::new(IoError::new(ErrorKind::InvalidInput, "Invalid scheme. Expected 'itlg'")));
    }

    let host = parsed_url.host_str().ok_or("No host specified")?.to_string();
    let port = parsed_url.port().unwrap_or(5050);
    let keeper = parsed_url.username().to_string();
    let secret = parsed_url.password().ok_or("No password specified")?.to_string();
    let chrono = parsed_url.path_segments().and_then(|segments| segments.last()).ok_or("No database specified")?.to_string();

    Ok(ConnectionInfo {
        keeper,
        secret,
        host,
        port,
        chrono,
    })
}
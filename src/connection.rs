use url::Url;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::error::Error;

#[allow(dead_code)]
pub struct ConnectionInfo {
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: String,
    pub port: u16,
    pub database: Option<String>,
    pub options: HashMap<String, String>,
}

pub fn parse_itlg_url(url: &str) -> Result<ConnectionInfo, Box<dyn std::error::Error + Send + Sync>> {
    let parsed_url = Url::parse(url)?;

    if parsed_url.scheme() != "itlg" {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid scheme. Expected 'itlg'")));
    }

    let host = parsed_url.host_str().ok_or("No host specified")?.to_string();
    let port = parsed_url.port().unwrap_or(5050);
    let username = parsed_url.username().to_string();
    let username = if username.is_empty() { None } else { Some(username) };
    let password = parsed_url.password().map(|s| s.to_string());
    let database = parsed_url.path_segments().and_then(|segments| segments.last()).map(|s| s.to_string());
    
    let options: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();

    Ok(ConnectionInfo {
        username,
        password,
        host,
        port,
        database,
        options,
    })
}


const CONFIG_FILE_PATH: &str = "~/.itlg/config";

fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        let home = std::env::var("HOME").unwrap();
        let path = path.strip_prefix("~/").unwrap();
        Path::new(&home).join(path)
    } else {
        PathBuf::from(path)
    }
}

pub fn validate_credentials(username: &str, password: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let config_path = expand_tilde(CONFIG_FILE_PATH);
    let file = File::open(&config_path)?;

    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 && parts[0] == username && parts[1] == password {
            return Ok(true);
        }
    }

    Ok(false)
}
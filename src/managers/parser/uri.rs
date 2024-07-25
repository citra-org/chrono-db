use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

pub struct ConnectionInfo {
    pub keeper: String,
    pub secret: String,
    pub host: String,
    pub port: u16,
    pub chrono: String,
}

pub fn parse_itlg_uri(uri: &str) -> Result<ConnectionInfo, Box<dyn Error + Send + Sync>> {
    if !uri.starts_with("itlg://") {
        return Err(Box::new(IoError::new(
            ErrorKind::InvalidInput,
            "Invalid scheme. Expected 'itlg'",
        )));
    }

    let uri = &uri[7..];
    let (user_info, host_info) = uri
        .split_once('@')
        .ok_or_else(|| IoError::new(ErrorKind::InvalidInput, "Invalid URI format"))?;
    let (keeper, secret) = user_info
        .split_once(':')
        .ok_or_else(|| IoError::new(ErrorKind::InvalidInput, "Invalid user info format"))?;

    let (host, chrono) = host_info
        .split_once('/')
        .ok_or_else(|| IoError::new(ErrorKind::InvalidInput, "No database specified"))?;

    let (host, port) = match host.split_once(':') {
        Some((h, p)) => (
            h.to_string(),
            p.parse()
                .map_err(|_| IoError::new(ErrorKind::InvalidInput, "Invalid port"))?,
        ),
        None => (host.to_string(), 3141),
    };

    Ok(ConnectionInfo {
        keeper: keeper.to_string(),
        secret: secret.to_string(),
        host,
        port,
        chrono: chrono.to_string(),
    })
}

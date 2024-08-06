use crate::{ops, managers};
use std::io::{Error, ErrorKind, Write};
use std::net::TcpStream;

pub fn handle_command(
    stream: &mut TcpStream,
    received: &str,
    chrono: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let parts: Vec<&str> = received.split_whitespace().collect();
    if parts.is_empty() {
        let response_str = "Error: Empty command\n";
        stream.write_all(response_str.as_bytes())?;
        return Ok(response_str.to_string());
    }

    let response = match parts.as_slice() {
        ["INSERT", header, body, "INTO", stream_name] => {
            if managers::validate::command::validate_commands(received) {
                ops::write::events::write_events(
                    chrono,
                    stream_name,
                    vec![(header.to_string(), body.to_string())],
                ).map(|_| "OK".to_string())
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            } else {
                Err(Box::new(Error::new(ErrorKind::InvalidInput, "Invalid command format")) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
        ["SELECT", "*", "FROM", stream_name] => {
            if managers::validate::command::validate_commands(received) {
                ops::read::events::read_events(chrono, stream_name)
                    .map(|data| format!("{}", data))
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            } else {
                Err(Box::new(Error::new(ErrorKind::InvalidInput, "Invalid command format")) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
        ["CREATE", stream_name] => {
            if managers::validate::command::validate_commands(received) {
                ops::create::stream::create_stream(chrono, stream_name)
                    .map(|_| "OK".to_string())
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            } else {
                Err(Box::new(Error::new(ErrorKind::InvalidInput, "Invalid command format")) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
        _ => Err(Box::new(Error::new(ErrorKind::InvalidInput, "Unknown command")) as Box<dyn std::error::Error + Send + Sync>),
    };

    match response {
        Ok(response_str) => {
            stream.write_all(response_str.as_bytes())?;
            Ok(response_str)
        }
        Err(e) => {
            let error_str = format!("Error: {}\n", e);
            stream.write_all(error_str.as_bytes())?;
            Err(e)
        }
    }
}
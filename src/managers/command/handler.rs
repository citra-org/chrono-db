use crate::{ops,validate};
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

    println!("{:?}", parts[0]);
    let response = match parts[0] {
        "e" => {
            println!("Ending connection with client.");
            Ok("OK".to_string())
        }
        // "ck" => ops::create::keeper::create_keeper(Some(parts[1])).map(|_| "Keeper created".to_string()),
        "cc" => ops::create::chrono::create_chrono(Some(chrono)).map(|_| "OK".to_string()),
        "cs" => ops::create::stream::create_stream(chrono, parts[1]).map(|_| "OK".to_string()),
        "w" => ops::write::events::write_events(
            chrono,
            parts[1],
            parts[2..]
                .chunks(2)
                .map(|chunk| (chunk[0].to_string(), chunk[1].to_string()))
                .collect(),
        )
        .map(|_| "OK".to_string()),
        "r" => ops::read::events::read_events(chrono, parts[1]).map(|data| format!("{}", data)),
        "ds" => ops::delete::stream::delete_stream(chrono, parts[1]).map(|_| "OK".to_string()),
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Unknown command: {}", parts[0]),
        )),
    };

    match response {
        Ok(response_str) => {
            stream.write_all(response_str.as_bytes())?;
            Ok(response_str)
        }
        Err(e) => {
            let error_str = format!("Error: {}\n", e);
            stream.write_all(error_str.as_bytes())?;
            Err(Box::new(e))
        }
    }
}

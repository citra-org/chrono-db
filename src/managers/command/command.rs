use std::io::{self, Write, Error, ErrorKind};
use std::net::TcpStream;
use crate::ops;

pub fn handle_command(stream: &mut TcpStream, received: &str, chrono: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let parts: Vec<&str> = received.split_whitespace().collect();
    if parts.is_empty() {
        let response_str = "Error: Empty command\n";
        stream.write_all(response_str.as_bytes())?;
        return Ok(response_str.to_string());
    }

    let response = match parts[0] {
        "e" => {
            println!("Ending connection with client.");
            Ok("Connection ended".to_string())
        }
        "ck" => ops::create::keeper::create_keeper(Some(parts[2])).map(|_| "Keeper created".to_string()),
        "cc" => ops::create::chrono::create_chrono(Some(chrono)).map(|_| "Chrono created".to_string()),
        "cs" => ops::create::stream::create_stream(Some(parts[2])).map(|_| "Stream created".to_string()),
        "w" => ops::write::write::write_events(
            "chrono", 
            "stream", 
            parts[1..].chunks(2).map(|chunk| (chunk[0].to_string(), chunk[1].to_string())).collect()
        ).map(|_| "Events written".to_string()),
        "r" => ops::read::read::read_events("hehe.itlg").map(|data| format!("{}", data)),
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Unknown command: {}", parts[0])
        )),
    };

    match response {
        Ok(response_str) => {
            println!("resp {}", response_str);
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

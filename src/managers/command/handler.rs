use crate::{managers, ops};
use std::io::{Error, ErrorKind, Write};
use std::net::TcpStream;

pub fn handle_command(
    stream: &mut TcpStream,
    received: &str,
    chrono: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    println!("【 received 】==> {:?}", received);
    if !managers::validate::command::validate_commands(chrono, received) {
        let error_str = "Error: Invalid command format\n";
        stream.write_all(error_str.as_bytes())?;
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "Invalid command format",
        )) as Box<dyn std::error::Error + Send + Sync>);
    }
    let mut parts = Vec::new();

    if received.starts_with("INSERT") {
        let mut split = received.split_whitespace();

        parts.extend(split.by_ref().take(4));

        if let Some(part4) = parts.get(3) {
            let rest_start = received.find(part4).unwrap() + part4.len();
            let rest = &received[rest_start..].trim();
            parts.push(rest);
        }
    } else {
        parts.extend(received.split_whitespace());
    }

    if parts.is_empty() {
        let response_str = "Error: Empty command\n";
        stream.write_all(response_str.as_bytes())?;
        return Ok(response_str.to_string());
    }
    println!("【 parts 】==> {:?}", parts);

    let response = match parts.as_slice() {
        ["INSERT", "INTO", stream_name, "VALUES", data] => {
            println!("【 data 】==> {:?}", data);

            let converted = data
                .trim_matches(|c| c == '(' || c == ')')
                .replace('\'', "")
                .split("), (")
                .map(|s| s.split(", ").collect::<Vec<&str>>())
                .map(|v| (v[0].to_string(), v[1].to_string()))
                .collect::<Vec<(String, String)>>();

            println!("【 converted 】==> {:?}", converted);
            ops::write::events::write_events(chrono, stream_name, converted)
                .map(|_| "OK".to_string())
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        }
        ["SELECT", "*", "FROM", stream_name] => ops::read::events::read_events(chrono, stream_name)
            .map(|data| format!("{}", data))
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        ["PING"] => Ok(String::from("PONG")),
        ["CREATE", "STREAM", stream_name] => {
            ops::create::stream::create_stream(chrono, stream_name)
                .map(|_| "OK".to_string())
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        }
        _ => Err(
            Box::new(Error::new(ErrorKind::InvalidInput, "Unknown command"))
                as Box<dyn std::error::Error + Send + Sync>,
        ),
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

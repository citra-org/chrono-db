use crate::{managers, ops};
use std::io::{Error, ErrorKind, Write};
use std::net::TcpStream;

pub fn handle_command(
    stream: &mut TcpStream,
    received: &str,
    chrono: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // INSERT INTO stream VALUES ('header1', 'body1'), ('header2', 'body2'), ('header3', 'body3')
    // INSERT INTO stream VALUES ('header1', 'body1')

    if !managers::validate::command::validate_commands(chrono, received) {
        let error_str = "Error: Invalid command format\n";
        stream.write_all(error_str.as_bytes())?;
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "Invalid command format",
        )) as Box<dyn std::error::Error + Send + Sync>);
    }

    let parts: Vec<&str> = if received.starts_with("INSERT") {
        let split_parts: Vec<&str> = received.split_whitespace().collect();
        let middle_body_start = split_parts[0].len();
        let middle_body_end = received.len()
            - (split_parts[split_parts.len() - 2].len()
                + split_parts[split_parts.len() - 1].len()
                + 2);
        let middle_body = &received[middle_body_start..middle_body_end].trim();
        vec![
            split_parts[0],
            middle_body,
            split_parts[split_parts.len() - 2],
            split_parts[split_parts.len() - 1],
        ]
    } else {
        received.split_whitespace().collect()
    };

    if parts.is_empty() {
        let response_str = "Error: Empty command\n";
        stream.write_all(response_str.as_bytes())?;
        return Ok(response_str.to_string());
    }
    println!("【 parts 】==> {:?}", parts);

    let response = match parts.as_slice() {
        ["INSERT", "INTO", stream_name, "VALUES", data] => {
            println!("【 data 】==> {:?}", data);

            let pairs_str = data.trim_matches(|c| c == '{' || c == '}').split("),(");
            println!("【 pairs_str 】==> {:?}", pairs_str);

            let events: Vec<(String, String)> = pairs_str
                .map(|pair| {
                    let pair = pair.trim_matches(|c| c == '(' || c == ')');
                    let mut iter = pair
                        .split(',')
                        .map(|s| s.trim().trim_matches('"').to_string());
                    (iter.next().unwrap(), iter.next().unwrap())
                })
                .collect();

            println!("【 events 】==> {:?}", events);
            ops::write::events::write_events(
                chrono,
                stream_name,
                events
                    .into_iter()
                    .map(|(header, body)| (header.to_string(), body.to_string()))
                    .collect(),
            )
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

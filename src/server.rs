use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write,Error,ErrorKind};
use crate::ops;
use crate::connection::validate_credentials;

pub fn run_server(host: &str, port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;
    println!("Server listening on {}:{}", host, port);

    for stream in listener.incoming() {
        println!("Received incoming connection");
        let stream = stream?;
        handle_client(stream)?;
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = [0; 10 * 1024];

    let n = stream.read(&mut buffer)?;
    if n == 0 {
        println!("Client disconnected");
        return Ok(());
    }

    let received = String::from_utf8_lossy(&buffer[..n]);
    println!("Received credentials: {}", received);

    let parts: Vec<&str> = received.split_whitespace().collect();
    if parts.len() < 3 {
        let response_str = "Error: Usage: <username> <password> <command>\n";
        stream.write_all(response_str.as_bytes())?;
        return Ok(());
    }

    let username = parts[0];
    let password = parts[1];
    // let command = parts[2];

    match validate_credentials(username, password) {
        Ok(valid) if valid => {
            stream.write_all(b"OK\n")?;
        }
        Ok(_) => {
            stream.write_all(b"Error: Invalid credentials\n")?;
            return Ok(());
        }
        Err(e) => {
            stream.write_all(format!("Error: {}\n", e).as_bytes())?;
            return Ok(());
        }
    }

    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            println!("Client disconnected");
            return Ok(());
        }

        let received = String::from_utf8_lossy(&buffer[..n]);
        println!("Received command: {}", received);

        let parts: Vec<&str> = received.split_whitespace().collect();
        if parts.is_empty() {
            let response_str = "Error: Empty command\n";
            stream.write_all(response_str.as_bytes())?;
            continue;
        }
        let response: Result<String,Error> = match parts[0] {
            "e" => { println!("Ending connection with client."); Ok("Connection ended".to_string()) },
            "ck" => ops::create::keeper::create_keeper(Some(parts[2])).map(|_| "Keeper created".to_string()),
            "cc" => ops::create::chrono::create_chrono(Some(parts[2])).map(|_| "Chrono created".to_string()),
            "cs" => ops::create::stream::create_stream(Some(parts[2])).map(|_| "Stream created".to_string()),
            "w" => ops::write::write::write_events("chrono", "stream", parts[1..].chunks(2).map(|chunk| (chunk[0].to_string(), chunk[1].to_string())).collect()).map(|_| "Events written".to_string()),
            "r" => ops::read::read::read_events("hehe.itlg").map(|events| format!("Read events: {:?}", events)),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unknown command: {}", parts[0])
            )),
        };
        
        let response_str = response.map_or_else(|e| format!("Error: {}", e), |data| data);
        stream.write_all(response_str.as_bytes())?;
    }
}

use crate::managers;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(
    mut stream: TcpStream,
    chrono: &str
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = [0; 10 * 1024];

    let n = stream.read(&mut buffer)?;
    if n == 0 {
        println!("Client disconnected");
        return Ok(());
    }

    let received = String::from_utf8_lossy(&buffer[..n]);
    println!("Received credentials:{}", received);

    let parts: Vec<&str> = received.split_whitespace().collect();
    if parts.len() < 3 {
        let response_str = "Error: Usage: <username> <password> <command>\n";
        stream.write_all(response_str.as_bytes())?;
        stream.flush()?;
        return Ok(());
    }
    let command = parts[0];
    let keeper = parts[1];
    let secret = parts[2];

    if command !="auth"{
        let response_str = "Invalid, need to auth.\n";
        stream.write_all(response_str.as_bytes())?;
        stream.flush()?;
    }


    let response = managers::identity::validate::validate_keeper(chrono, keeper, secret)?;
    if response != "OK" {
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
        return Ok(());
    }

    let response_str = "Connection established. Send your command.\n";
    stream.write_all(response_str.as_bytes())?;
    stream.flush()?;

    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            println!("Client disconnected");
            return Ok(());
        }

        let received = String::from_utf8_lossy(&buffer[..n]);
        println!("Received command: {}", received);

        managers::command::handler::handle_command(&mut stream, &received, chrono)?;
    }
}

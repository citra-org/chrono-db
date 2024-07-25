use crate::managers;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(
    mut stream: TcpStream,
    chrono: &str,
    keeper: &str,
    secret: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
        stream.flush()?;
        return Ok(());
    }

    let response = managers::identity::validate::validate_keeper(keeper, secret)?;
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            println!("Client disconnected");
            return Ok(());
        }

        let received = String::from_utf8_lossy(&buffer[..n]);
        println!("Received command: {}", received);

        managers::command::command::handle_command(&mut stream, &received, chrono)?;
    }
}

use crate::managers;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::cmp;


pub fn handle_client(
    mut stream: TcpStream,
    chrono: &str,
    // client_number: usize,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let buffer_size = get_optimal_buffer_size();
    let mut buffer = vec![0u8; buffer_size];
    //TODO: Improve logic for buffer size

    let n = stream.read(&mut buffer)?;
    if n == 0 {
        println!("Client disconnected");
        return Ok(());
    }

    let received = String::from_utf8_lossy(&buffer[..n]);
    println!("Received credentials: {}", received);

    println!(":::{}:::{}",received,chrono);
    let parts: Vec<&str> = received.split_whitespace().collect();
    if parts.len() < 3 {
        let response_str = "Error: Usage: <username> <password> <command>\n";
        stream.write_all(response_str.as_bytes())?;
        stream.flush()?;
        return Ok(());
    }

    let response = managers::identity::validate::validate_keeper(chrono, parts[1], parts[2])?;
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    loop {
        let buffer_size = get_optimal_buffer_size();
        let mut buffer = vec![0u8; buffer_size];

        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                return Ok(());
            }
            Ok(n) => {
                let received = String::from_utf8_lossy(&buffer[..n]);
                println!("Received command: {}", received);
                if let Err(e) = managers::command::handler::handle_command(&mut stream, &received, chrono) {
                    eprintln!("Error handling command: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                return Err(e.into());
            }
        }
    }

}

fn get_optimal_buffer_size() -> usize {
    const DEFAULT_BUFFER_SIZE: usize = 1024 * 1024; 
    #[cfg(unix)]
    {
        use libc::{sysconf, _SC_PAGESIZE};
        let page_size = unsafe { sysconf(_SC_PAGESIZE) as usize };
        if page_size > 0 {
            return cmp::max(page_size, DEFAULT_BUFFER_SIZE);
        }
    }

    DEFAULT_BUFFER_SIZE
}
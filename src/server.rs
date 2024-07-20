use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::ops;

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
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    let received = String::from_utf8_lossy(&buffer[..n]);
    
    println!("Received command: {}", received);
    
    let parts: Vec<&str> = received.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".into());
    }
    
    let file_name = &format!("{}.itlg", parts.get(1).unwrap_or(&"default"));
    
    let response = match parts[0] {
        "g" => ops::generate::generate_user(),
        "c" => ops::create::create_record(file_name),
        "w" => {
            let chunks: Vec<(String, String)> = parts[2..].chunks(2)
                .map(|chunk| (chunk[0].to_string(), chunk[1].to_string()))
                .collect();
            ops::write::write_record(file_name, chunks)
        },
        "r" => ops::read::read_records(file_name),
        _ => Err(format!("Unknown command: {}", parts[0]).into()),
    };
    
    let response_str = match response {
        Ok(_) => "OK\n".to_string(),
        Err(e) => format!("Error: {}\n", e),
    };
    
    stream.write_all(response_str.as_bytes())?;
    
    Ok(())
}
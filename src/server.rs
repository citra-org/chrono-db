use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
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
    let mut buffer = [0; 1024];
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
            let response_str = "Error: Empty command\n".to_string();
            stream.write_all(response_str.as_bytes())?;
            continue;
        }

        let response = match parts[0] {
            "e" => {
                println!("Ending connection with client.");
                break;
            }
            "g" => execute_command(parts),
            _ => {
                if parts.len() < 3 {
                    Err("Usage: <command> <username> <password> <args>".into())
                } else {
                    let username = parts[1];
                    let password = parts[2];
                    match validate_credentials(username, password) {
                        Ok(valid) if valid => execute_command(parts),
                        Ok(_) => Err("Invalid credentials".into()),
                        Err(e) => Err(e),
                    }
                }
            }
        };
        
        let response_str = match response {
            Ok(_) => "OK\n".to_string(),
            Err(e) => format!("Error: {}\n", e),
        };
        
        stream.write_all(response_str.as_bytes())?;
    }
    
    Ok(())
}

fn execute_command(parts: Vec<&str>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match parts[0] {
        "g" => ops::generate::generate_user(),
        "c" => ops::create::create_record("hehe.itlg"),
        "w" => {
            println!("writing..");
            let chunks: Vec<(String, String)> = parts[3..].chunks(2)
                .map(|chunk| (chunk[0].to_string(), chunk[1].to_string()))
                .collect();
            ops::write::write_record("hehe.itlg", chunks)
        },
        "r" => {
            println!("reading..");
            let file_name = &format!("{}.itlg", parts.get(3).unwrap_or(&"default"));
            ops::read::read_records("hehe.itlg");
            Ok(())
        },
        _ => Err(format!("Unknown command: {}", parts[0]).into()),
    }
}

use crate::client;
use ini::Ini;
use std::net::TcpListener;

pub fn run_server(chrono: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conf =
        Ini::load_from_file("conf.ini").map_err(|e| format!("Failed to load INI file: {}", e))?;

    let section = conf
        .section(Some("CHRONO_DB"))
        .ok_or("Section 'CHRONO_DB' not found in the INI file")?;

    let chrono_host = section
        .get("HOST")
        .ok_or("HOST not found in 'CHRONO_DB' section")?;
    let chrono_port = section
        .get("PORT")
        .ok_or("PORT not found in 'CHRONO_DB' section")?;

    println!("Starting server on {}:{}", chrono_host, chrono_port);

    let listener = TcpListener::bind(format!("{}:{}", chrono_host, chrono_port))?;
    println!("Server listening on {}:{}", chrono_host, chrono_port);

    for stream in listener.incoming() {
        println!("Received incoming connection");
        let stream = stream?;
        client::conn::handle_client(stream, chrono)?;
    }
    Ok(())
}

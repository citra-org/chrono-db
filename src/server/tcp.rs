use std::net::TcpListener;
use crate::client;

pub fn run_server(host: &str, port: u16, chrono:&str, keeper:&str, secret:&str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;
    println!("Server listening on {}:{}", host, port);

    for stream in listener.incoming() {
        println!("Received incoming connection");
        let stream = stream?;
        client::conn::handle_client(stream, chrono,keeper, secret)?;
    }
    Ok(())
}

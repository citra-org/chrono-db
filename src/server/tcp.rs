use crate::client;
use ini::Ini;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

static CLIENT_COUNTER: AtomicUsize = AtomicUsize::new(0);

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
        let stream = stream?;
        let client_number = CLIENT_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
        print_client_details(&stream, client_number);

        let chrono = chrono.to_string(); // Clone chrono for the new thread
        thread::spawn(move || {
            if let Err(e) = client::conn::handle_client(stream, &chrono) {
                eprintln!("Error handling client {}: {}", client_number, e);
            }
            CLIENT_COUNTER.fetch_sub(1, Ordering::SeqCst);
            println!(
                "Client {} disconnected. Total active clients: {}",
                client_number,
                CLIENT_COUNTER.load(Ordering::SeqCst)
            );
        });
    }
    Ok(())
}

fn print_client_details(stream: &TcpStream, client_number: usize) {
    if let Ok(addr) = stream.peer_addr() {
        println!("New client connected:");
        println!("  Client Number: {}", client_number);
        println!("  IP Address: {}", addr.ip());
        println!("  Port: {}", addr.port());
        println!(
            "  Total active clients: {}",
            CLIENT_COUNTER.load(Ordering::SeqCst)
        );
    } else {
        println!("New client connected (unable to get address details)");
        println!("  Client Number: {}", client_number);
        println!(
            "  Total active clients: {}",
            CLIENT_COUNTER.load(Ordering::SeqCst)
        );
    }
}

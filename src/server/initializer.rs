use crate::{managers, ops, server};
use std::env;
use std::io::Result;
use std::path::Path;

fn print_usage() {
    println!("Usage:");
    println!("  init <chrono> <keeper>");
    println!("  start <chrono>");
    println!("  stop <chrono>");
}

pub fn initializer() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "init" => {
            let chrono = &args[2];
            let keeper = &args[3];
            println!("{} ::: {}", keeper, chrono);
            ops::create::keeper::create_keeper(chrono, Some(keeper)).map_err(|e| {
                eprintln!("Error creating keeper: {}", e);
                e
            })?;
            ops::create::chrono::create_chrono(Some(chrono)).map_err(|e| {
                eprintln!("Error creating chrono: {}", e);
                e
            })?;
        }
        "start" => {
            let chrono = &args[2];
            if let Err(e) = managers::folders::check::check_folder(chrono, true) {
                eprintln!("Chrono doesn't exist: {}", e);
            } else {
                println!("Starting server for database: {}", chrono);
                if let Err(e) = server::tcp::run_server(chrono).map_err(|e| {
                    eprintln!("Error running server: {:?}", e);
                    e
                }) {
                    eprintln!("Server failed: {}", e);
                }
            }
        }
        "stop" => {
            let chrono = &args[2];
            println!("Stopping server for database: {}", chrono);
            // server::tcp::stop_server();
        }
        _ => {
            print_usage();
        }
    }

    Ok(())
}

use crate::{ops, server};
use std::env;
use std::io::Result;
use std::path::Path;

fn print_usage() {
    println!("Usage:");
    println!("  init <database name> <username>");
    println!("  start <database name>");
    println!("  stop <database name>");
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
            let path = Path::new(chrono);

            if !path.exists() {
                eprintln!("Chrono Doesnt exsist");
            } else {
                println!("Starting server for database: {}", chrono);
                let _ = server::tcp::run_server(chrono).map_err(|e| {
                    eprintln!("Error running server: {:?}", e);
                    e
                });
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

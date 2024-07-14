use std::env;
mod ops {
    pub mod read;
    pub mod write;
    pub mod create;
}

mod helper {
    pub mod crypto;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run [create|write|read]");
        return Ok(());
    }
    match args[1].as_str() {
        "create" => ops::create::create_key_and_sample_file()?,
        "write" => {
                let headers = args[2].clone();
                let body = args[3].clone();
                ops::write::write_record(headers, body)?;
            
        },
        "read" => {
                ops::read::read_records()?;
            
        },
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }

    Ok(())
}
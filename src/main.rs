mod ops {
    pub mod read;
    pub mod write;
}

mod helper {
    pub mod crypto;
}

use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    // This should be securely stored and retrieved in a real application
    let key: [u8; 32] = [1; 32];  // Example key, don't use this in production!

    if args.len() < 3 {
        eprintln!("Usage: {} [write <filename> <headers> <body> | read <filename>]", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "write" => {
            if args.len() != 5 {
                eprintln!("Usage: {} write <filename> <headers> <body>", args[0]);
            } else {
                let filename = format!("{}.itlg", args[2]);
                let headers = args[3].clone();
                let body = args[4].clone();
                ops::write::write_record(&filename, headers, body, &key)?;
            }
        },
        "read" => {
            if args.len() != 3 {
                eprintln!("Usage: {} read <filename>", args[0]);
            } else {
                let filename = format!("{}.itlg", args[2]);
                ops::read::read_records(&filename, &key)?;
            }
        },
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }

    Ok(())
}
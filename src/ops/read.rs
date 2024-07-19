use std::fs::File;
use std::io::{self, BufRead};


pub fn read_records(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.splitn(3, ' ');

        if let (Some(time_str), Some(body), Some(headers)) = (parts.next(), parts.next(), parts.next()) {
            println!("{} {} {}", time_str, body, headers);
        } else {
            eprintln!("Malformed record: {}", line);
        }
    }
    
    Ok(())
}

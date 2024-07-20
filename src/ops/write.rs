use std::fs::OpenOptions;
use std::io::{Write, Result as IoResult};
use std::time::Instant;

pub fn write_record(file_path: &str, records: Vec<(String, String)>) -> IoResult<()> {
    let start_time = Instant::now(); 

    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)?;
    
    let mut buffer = String::new();
    for (header, body) in &records {
        let combined = format!("{} {} {}\n", "2024-07-20 20:10:44.237606774 UTC", body, header); 
        buffer.push_str(&combined);
    }
    
    file.write_all(buffer.as_bytes())?;
    
    let duration = start_time.elapsed(); 

    println!("time taken: {:?}", duration); 
    println!("written i: {}", records.len()); 
    println!("write speed: {} i/s", records.len() as f64 / duration.as_secs_f64()); // Print the write speed

    Ok(())
}

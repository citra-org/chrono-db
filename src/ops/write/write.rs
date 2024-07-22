use std::io::{BufWriter, Write};
use std::fs::OpenOptions;
use std::sync::Arc;
use std::time::Instant;
use chrono::Utc;
use std::thread;
use num_cpus;

pub fn write_record(file_path: &str, records: Vec<(String, String)>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start_time = Instant::now();
    let file_path = Arc::new(file_path.to_string());
    let records = Arc::new(records);
    let num_cpus = num_cpus::get();
    let records_per_thread = (records.len() + num_cpus - 1) / num_cpus;
    
    let mut handles = vec![];
    for i in 0..num_cpus {
        let file_path = file_path.clone();
        let records = records.clone();
        let handle = thread::spawn(move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let start = i * records_per_thread;
            let end = std::cmp::min((i + 1) * records_per_thread, records.len());
            
            if start >= records.len() {
                return Ok(());
            }
            
            let file = OpenOptions::new()
                .append(true)
                .open(&*file_path)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            let mut buf_writer = BufWriter::with_capacity(8192, file);
            for (header, body) in records[start..end].iter() {
                let time = Utc::now();
                let combined = format!("{} {} {}\n", time, body, header);
                buf_writer.write_all(combined.as_bytes())
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            }
            buf_writer.flush()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            Ok(())
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap()?;
    }
    
    let duration = start_time.elapsed();
    println!("CPU's: {}", num_cpus);
    println!("Time taken: {:?}", duration);
    println!("Records written: {}", records.len());
    println!("Write speed: {} records/second", records.len() as f64 / duration.as_secs_f64());
    Ok(())
}
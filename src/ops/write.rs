use std::io::{BufWriter, Write, Result, Error, ErrorKind};
use std::fs::OpenOptions;
use std::sync::Arc;
use std::time::Instant;
use chrono::Utc;
use std::thread;
use num_cpus;

pub fn write_record(file_path: &str, records: Vec<(String, String)>) -> Result<()> {
    let start_time = Instant::now();
    let file_path = Arc::new(file_path.to_string());
    let records = Arc::new(records);
    let num_cpus = num_cpus::get();
    let records_per_thread = (records.len() + num_cpus - 1) / num_cpus;

    let mut handles = vec![];
    for i in 0..num_cpus {
        let file_path = file_path.clone();
        let records = records.clone();
        let handle = thread::spawn(move || {
            let start = i * records_per_thread;
            let end = std::cmp::min((i + 1) * records_per_thread, records.len());
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&*file_path)
                .map_err(|e| Error::new(ErrorKind::PermissionDenied, format!("Failed to open file: {}", e)))?;
            
            let mut buf_writer = BufWriter::with_capacity(8192, file);
            for (header, body) in records[start..end].iter() {
                let time = Utc::now();
                let combined = format!("{} {} {}\n", time, body, header);
                buf_writer.write_all(combined.as_bytes())
                    .map_err(|e| Error::new(ErrorKind::WriteZero, format!("Failed to write data into file: {}", e)))?;
            }
            buf_writer.flush()
                .map_err(|e| Error::new(ErrorKind::WriteZero, format!("Failed to flush buffer: {}", e)))?;
            Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join().unwrap() {
            return Err(e);
        }
    }

    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);
    println!("Records written: {}", records.len());
    println!("Write speed: {} records/second", records.len() as f64 / duration.as_secs_f64());

    Ok(())
}
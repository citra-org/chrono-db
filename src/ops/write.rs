use std::fs::{OpenOptions, File};
use std::io::{BufWriter, Write, Result, Error, ErrorKind};
use std::sync::Arc;
use std::thread;
use chrono::Utc;

const CHUNK_SIZE: usize = 10000;

pub fn write_record(file_path: &str, records: Vec<(String, String)>) -> Result<()> {
    let record_chunks: Vec<Vec<(String, String)>> = records.chunks(CHUNK_SIZE).map(|chunk| chunk.to_vec()).collect();
    let file_path = Arc::new(file_path.to_string());

    let mut handles = vec![];

    for chunk in record_chunks {
        let file_path = file_path.clone();
        let chunk = chunk.clone();

        let handle = thread::spawn(move || {
            let file = OpenOptions::new().append(true).create(true).open(&*file_path)
                .map_err(|e| Error::new(ErrorKind::PermissionDenied, format!("Failed to open file: {}", e)))?;
            
            let mut buf_writer = BufWriter::new(file);

            for (header, body) in chunk.iter() {
                let time = Utc::now();
                let combined = format!("{} {} {}\n", time, body, header);
                let combined_bytes = combined.as_bytes();

                buf_writer.write_all(combined_bytes)
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

    Ok(())
}

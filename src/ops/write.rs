use std::fs::OpenOptions;
use chrono::Utc;
use std::io::{Write, Result, Error, ErrorKind};

pub fn write_record(file_path: &str, records: Vec<(String, String)>) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .map_err(|e| Error::new(ErrorKind::PermissionDenied, format!("Failed to open file: {}", e)))?;

    for (header, body) in records {
        let time = Utc::now();
        let combined = format!("{} {} {}\n", time, body, header);
        let combined_bytes = combined.as_bytes();

        file.write_all(&combined_bytes)
            .map_err(|e| Error::new(ErrorKind::WriteZero, format!("Failed to write data into file: {}", e)))?;
    }

    Ok(())
}

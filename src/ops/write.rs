use std::fs::{OpenOptions};
use std::io::{self, Write, Read, Seek, SeekFrom};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use bincode;
use crate::helper::crypto;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataRecord {
    pub time: DateTime<Utc>,
    pub headers: String,
    pub body: String,
}

impl DataRecord {
    pub fn new(headers: String, body: String) -> Self {
        Self {
            time: Utc::now(),
            headers,
            body,
        }
    }
}

pub fn write_record(filename: &str, headers: String, body: String, key: &[u8; 32]) -> io::Result<()> {
    let record = DataRecord::new(headers, body);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;

    let encoded: Vec<u8> = bincode::serialize(&record).unwrap();
    let encrypted = crypto::encrypt(&encoded, key);
    let new_record = (encrypted.len() as u32).to_be_bytes().to_vec();
    let new_record = [new_record, encrypted].concat();

    // Read existing content
    let mut existing_content = Vec::new();
    file.read_to_end(&mut existing_content)?;

    // Move file cursor to the beginning
    file.seek(SeekFrom::Start(0))?;

    // Write new record followed by existing content
    file.write_all(&new_record)?;
    file.write_all(&existing_content)?;
    file.set_len(new_record.len() as u64 + existing_content.len() as u64)?;
    file.flush()?;

    Ok(())
}
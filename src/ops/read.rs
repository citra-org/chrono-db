use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
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

pub fn read_records(filename: &str, key: &[u8; 32]) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let file_len = file.metadata()?.len();
    let mut position = 0;

    while position < file_len {
        file.seek(SeekFrom::Start(position))?;
        let mut reader = BufReader::new(&file);
        let mut size_buffer = [0u8; 4];

        reader.read_exact(&mut size_buffer)?;
        let size = u32::from_be_bytes(size_buffer) as usize;

        let mut encrypted_buffer = vec![0u8; size];
        reader.read_exact(&mut encrypted_buffer)?;

        if let Some(decrypted) = crypto::decrypt(&encrypted_buffer, key) {
            match bincode::deserialize::<DataRecord>(&decrypted) {
                Ok(record) => println!("{:?}", record),
                Err(e) => eprintln!("Error deserializing record: {}", e),
            }
        } else {
            eprintln!("Error decrypting record");
        }

        position += 4 + size as u64;
    }

    Ok(())
}
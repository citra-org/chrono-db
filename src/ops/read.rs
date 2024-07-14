use std::fs::File;
use std::io::{Read};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use crate::helper::crypto::read_key; 
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataRecord {
    pub time: DateTime<Utc>,
    pub headers: String,
    pub body: String,
}
impl DataRecord {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&[u8]> = bytes.splitn(3, |&b| b == b' ').collect();
        if parts.len() != 3 {
            return Err("Invalid record format".into());
        }

        let time_str = std::str::from_utf8(parts[0])?;
        let time = DateTime::parse_from_rfc3339(time_str)?.with_timezone(&Utc);
        let headers = String::from_utf8(parts[1].to_vec())?;
        let body = String::from_utf8(parts[2].to_vec())?;

        Ok(Self { time, headers, body })
    }
}

pub fn read_records() -> Result<(), Box<dyn std::error::Error>> {
    let key = read_key()?;
    let mut file = File::open("sample.itlg")?;
    let mut iv = [0u8; 16];
    file.read_exact(&mut iv)?;
    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)?;

    let cipher = Aes256Cbc::new_from_slices(&key, &iv)?;
    let decrypted_bytes = cipher.decrypt_vec(&ciphertext)?;

    let records: Vec<DataRecord> = decrypted_bytes
        .split(|&b| b == b'\n')
        .filter(|&record| !record.is_empty())
        .map(DataRecord::from_bytes)
        .collect::<Result<_, _>>()?;

    for (i, record) in records.iter().enumerate() {
        println!("Decrypted Record {}:", i + 1);
        println!("Time: {}", record.time);
        println!("Headers: {}", record.headers);
        println!("Body: {}", record.body);
        println!();
    }

    Ok(())
}

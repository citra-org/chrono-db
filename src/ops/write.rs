use std::io::{Write};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use crate::helper::crypto::read_key;
use std::fs::OpenOptions;
use aes::Aes256;
use rand::Rng;
use block_modes::{Cbc};
use block_modes::block_padding::Pkcs7;
use block_modes::BlockMode;
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

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

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.time.to_rfc3339().as_bytes());
        bytes.extend(&[b' ']);
        bytes.extend(self.headers.as_bytes());
        bytes.extend(&[b' ']);
        bytes.extend(self.body.as_bytes());
        bytes
    }
}

pub fn write_record(headers: String, body: String) -> Result<(), Box<dyn std::error::Error>> {
    let key = read_key()?;
    let iv: [u8; 16] = rand::thread_rng().gen();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv)?;
    let record = DataRecord::new(headers, body);
    let mut plaintext = record.as_bytes();
    plaintext.push(b'\n'); 
    let ciphertext = cipher.encrypt_vec(&plaintext);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("sample.itlg")?;

    file.write_all(&iv)?;
    file.write_all(&ciphertext)?;

    Ok(())
}
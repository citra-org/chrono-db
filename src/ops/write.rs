use std::io::Write;
use chrono::prelude::*;
use crate::helper::crypto::read_key;
use std::fs::OpenOptions;
use aes::Aes256;
use rand::Rng;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn write_record(headers: String, body: String) -> Result<(), Box<dyn std::error::Error>> {

    let key = read_key()?;
    let iv: [u8; 16] = rand::thread_rng().gen();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv)?;
    
    let time = Utc::now();

    let combined = format!("{} {} {}",time, body, headers);
    let combined_bytes = combined.as_bytes();

    let ciphertext = cipher.encrypt_vec(combined_bytes);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("sample.itlg")?;

    file.write_all(&iv)?;
    file.write_all(&ciphertext)?;
    Ok(())
}
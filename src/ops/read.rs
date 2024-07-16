use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use crate::helper::crypto::read_key; 
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn read_records() -> Result<(), Box<dyn std::error::Error>> {
    let key = read_key()?;
    // let mut file = File::open("sample.itlg")?;
    // file.read_exact(&mut iv)?;
    // file.read_to_end(&mut ciphertext)?;
    
    let file = File::open("sample.itlg")?;
    let reader = BufReader::new(file);
    
    for (index, line) in reader.lines().enumerate() {
        let mut iv = [0u8; 16];
        let line = line?;
        let cipher = Aes256Cbc::new_from_slices(&key, &iv)?;
        let decrypted_bytes = cipher.decrypt_vec(&line)?;
        let content = String::from_utf8(decrypted_bytes)?;
        println!("{}: {}", index + 1, content);
    }
    Ok(())
}

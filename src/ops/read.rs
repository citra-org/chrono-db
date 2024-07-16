use std::fs::File;
use std::io::{Read};
use crate::helper::crypto::read_key; 
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn read_records() -> Result<(), Box<dyn std::error::Error>> {
    let key = read_key()?;
    let mut file = File::open("sample.itlg")?;
    let mut iv = [0u8; 16];
    file.read_exact(&mut iv)?;
    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)?;

    let cipher = Aes256Cbc::new_from_slices(&key, &iv)?;
    let decrypted_bytes:Vec<u8> = cipher.decrypt_vec(&ciphertext)?;

    let string_data = String::from_utf8_lossy(&decrypted_bytes);
    println!("{}", string_data);

    Ok(())
}

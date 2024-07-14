use std::fs::File;
use rand::Rng;
use std::io::Write;
pub fn create_key_and_sample_file() -> Result<(), Box<dyn std::error::Error>> {
    let key: [u8; 32] = rand::thread_rng().gen();
    
    let key_base64 = base64::encode(&key);
    println!("Created key (base64): {}", key_base64);

    let mut file = File::create(".itlgs")?;
    file.write_all(&key)?;

    File::create("sample.itlg")?;
    println!("Key and sample.itlg files created.");
    Ok(())
}
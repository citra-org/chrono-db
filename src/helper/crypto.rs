use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce
};
use rand::Rng;

pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce_bytes = random_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, data).unwrap();
    [&nonce_bytes[..], &ciphertext].concat()
}

pub fn decrypt(encrypted_data: &[u8], key: &[u8; 32]) -> Option<Vec<u8>> {
    if encrypted_data.len() < 12 {
        return None;
    }
    let (nonce, ciphertext) = encrypted_data.split_at(12);
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    cipher.decrypt(Nonce::from_slice(nonce), ciphertext).ok()
}

fn random_nonce() -> [u8; 12] {
    let mut rng = OsRng;
    rng.gen()
}
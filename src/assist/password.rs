use crate::assist;

pub fn generate_random_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%&";
    let mut seed = assist::time::get_current_time(false) as u128;
    (0..length).map(|_| {
        seed = seed.wrapping_mul(0x5DEECE66D).wrapping_add(0xB);
        CHARSET[(seed >> 16) as usize % CHARSET.len()] as char
    }).collect()
}

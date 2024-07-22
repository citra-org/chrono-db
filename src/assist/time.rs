use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_time(time_type:bool) -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    match time_type {
        false => duration.as_nanos(),
        _ => duration.as_millis(),
    }
}
use crate::managers;
use std::sync::Arc;

pub fn validate_commands(command: &str) -> bool {
    let parts: Vec<&str> = command.split_whitespace().collect();
    
    fn is_stream_valid(name: &str) -> bool {
        is_stream_name_valid(name) && is_stream_exists(name)
    }

    fn is_stream_name_valid(name: &str) -> bool {
        !name.is_empty() 
        && name.chars().next().map_or(false, |c| c.is_ascii_lowercase() || c == '_')
        && name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && !name.starts_with('_')
        && !name.ends_with('_')
    }
    
    fn is_stream_exists(name: &str) -> bool {
        let file_path = Arc::new(format!(
            "/var/lib/citra/chrono/{}/{}.chrono",
            "chrono", name
        ));
        match managers::files::check::check_file(&file_path, true) {
            Ok(_) => true,
            Err(_) => false,
        }
    
    }

    match parts.as_slice() {
        ["INSERT", header, body, "INTO", stream] => {
            is_stream_valid(&stream.to_lowercase()) &&
            header.starts_with('"') && header.ends_with('"') &&
            body.starts_with('"') && body.ends_with('"')
        },
        ["SELECT", "*", "FROM", stream]  => {
            is_stream_valid(&stream.to_lowercase())
            // | ["SELECT", value, "FROM", stream]
            // (value == "*" || (value.starts_with('"') && value.ends_with('"')))
        },
        ["CREATE", stream] => {
            is_stream_valid(&stream.to_lowercase())
        },
        _ => false,
    }
}
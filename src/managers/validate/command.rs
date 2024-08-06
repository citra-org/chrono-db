use crate::managers;
use regex::Regex;
use std::sync::Arc;

pub fn validate_commands(parts: Vec<&str>) -> bool {
    match parts.as_slice() {
        ["INSERT", event, "INTO", stream] => {
            is_stream_valid(&stream.to_lowercase()) && is_event_valid(event)
        }
        ["SELECT", "*", "FROM", stream] => {
            is_stream_valid(&stream.to_lowercase())
            // | ["SELECT", value, "FROM", stream]
            // (value == "*" || (value.starts_with('"') && value.ends_with('"')))
        }
        ["CREATE", "STREAM", stream] => is_stream_name_valid(&stream.to_lowercase()),
        _ => false,
    }
}

fn is_stream_valid(name: &str) -> bool {
    is_stream_name_valid(name) && is_stream_exists(name)
}

fn is_stream_name_valid(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_lowercase() || c == '_')
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
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
fn is_event_valid(event: &str) -> bool {
    let single_tuple_pattern = r#"\(\s*"\w+"\s*,\s*"\w+"\s*\)"#;
    let multiple_tuples_pattern =
        r#"\{\s*(\(\s*"\w+"\s*,\s*"\w+"\s*\)\s*,\s*)*\(\s*"\w+"\s*,\s*"\w+"\s*\)\s*\}"#;

    let single_tuple_regex = Regex::new(single_tuple_pattern).unwrap();
    let multiple_tuples_regex = Regex::new(multiple_tuples_pattern).unwrap();

    single_tuple_regex.is_match(event) || multiple_tuples_regex.is_match(event)
}

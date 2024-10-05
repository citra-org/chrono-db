use crate::managers;
use regex::Regex;
use std::sync::Arc;

pub fn validate_commands(chrono: &str, input: &str) -> bool {
    let trimmed_input = input.trim_end_matches('\n').trim_end_matches("\n\n");
    let parts: Vec<&str> = trimmed_input.splitn(5, |c| c == ' ').collect();
    let chrono_lower = chrono.to_lowercase();

    match parts.as_slice() {
        ["INSERT", "INTO", stream, "VALUES", event] => {
            is_stream_valid(&chrono_lower, &stream.to_lowercase()) && is_event_valid(event)
        }
        ["SELECT", "*", "FROM", stream] => is_stream_valid(&chrono_lower, &stream.to_lowercase()),
        ["CREATE", "STREAM", stream] => is_stream_name_valid(&stream.to_lowercase()),
        ["PING"] => true,
        _ => false,
    }
}

fn is_stream_valid(chrono: &str, stream: &str) -> bool {
    is_stream_name_valid(stream) && is_stream_exists(chrono, stream)
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

fn is_stream_exists(chrono: &str, stream: &str) -> bool {
    let file_path = Arc::new(format!(
        "/var/lib/citra/chrono/{}/{}.chrono",
        chrono, stream
    ));
    match managers::files::check::check_file(&file_path, true) {
        Ok(_) => true,
        Err(_) => false,
    }
}
fn is_event_valid(event: &str) -> bool {
    println!("event: {:#?}", event);

    let pattern = r"^\(\'.+?\'\, \'.+?\'\)(\, \(\'.+?\'\, \'.+?\'\))*$";
    let re = Regex::new(pattern).unwrap();

    println!("re.is_match: {:#?}", re.is_match(event));
    println!(
        "is_balanced_parentheses: {:#?}",
        is_balanced_parentheses(event)
    );
    return re.is_match(event) && is_balanced_parentheses(event);
}

fn is_balanced_parentheses(event: &str) -> bool {
    let mut stack = Vec::new();
    for e in event.chars() {
        match e {
            '(' => stack.push(e),
            ')' => {
                if stack.pop().is_none() {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

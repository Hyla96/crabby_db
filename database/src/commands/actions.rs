use crate::memory::memory_database::MEMORY_DATABASE;
use crate::utils::parser::{find_next_whitespace, skip_whitespace};

pub async fn get(buffer: &[u8], index: usize) -> String {
    let start = skip_whitespace(buffer, index);
    if start >= buffer.len() {
        return "Error: Missing key for GET command\n".to_string();
    }

    let end = find_next_whitespace(buffer, start);

    let key = &buffer[start..end];

    match MEMORY_DATABASE.get(key) {
        Some(value) => format!(
            "GET command received: {}\n",
            String::from_utf8_lossy(&value)
        ),
        None => format!("GET command received: [404]\n"),
    }
}

pub async fn set(buffer: &[u8], index: usize) -> String {
    let start = skip_whitespace(buffer, index);
    if start >= buffer.len() {
        return "Error: Missing key for SET command\n".to_string();
    }

    let end = find_next_whitespace(buffer, start);

    let key = &buffer[start..end];

    let start = skip_whitespace(buffer, end);

    if start >= buffer.len() {
        return "Error: Missing value for SET command\n".to_string();
    }

    let end = find_next_whitespace(buffer, start);

    let value = &buffer[start..end];

    MEMORY_DATABASE.set(key, value);

    format!(
        "SET command received: [key:{}][value:{}]\n",
        String::from_utf8_lossy(key),
        String::from_utf8_lossy(value),
    )
}

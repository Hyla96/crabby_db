use crate::commands::actions::{get, set};
use crate::utils::parser::{find_next_whitespace, skip_whitespace};

pub async fn process_command(v: &[u8]) -> String {
    let index = skip_whitespace(v, 0);
    if index >= v.len() {
        return "Error: Empty command\n".to_string();
    }

    let end = find_next_whitespace(v, index);

    let cmd = &v[index..end];

    match cmd {
        b"set" | b"SET" => set(v, end).await,
        b"get" | b"GET" => get(v, end).await,
        b"delete" | b"DELETE" => "DELETE command received\n".to_string(),
        b"list" | b"LIST" => "LIST command received\n".to_string(),
        _ => "Error: Unknown command\n".to_string(),
    }
}

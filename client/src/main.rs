use std::io::{self, Write};
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Key/Value Database TCP Client");
    println!("Commands:");
    println!("  s - SET key value");
    println!("  g - GET key");
    println!("  d - DELETE key");
    println!("  l - LIST all keys");
    println!("  q - QUIT");
    println!("  h - HELP");
    println!();

    // Connect to the server
    let server_addr = "127.0.0.1:8080";
    println!("Connecting to {}...", server_addr);

    let mut stream = match TcpStream::connect(server_addr).await {
        Ok(stream) => {
            println!("Connected successfully!");
            stream
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            eprintln!(
                "Make sure the key/value server is running on {}",
                server_addr
            );
            return Ok(());
        }
    };

    loop {
        print!("kv> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let command = match input.chars().next().unwrap().to_lowercase().next().unwrap() {
            's' => handle_set_command(input),
            'g' => handle_get_command(input),
            'd' => handle_delete_command(input),
            'l' => handle_list_command(),
            'q' => {
                println!("Goodbye!");
                break;
            }
            'h' => {
                show_help();
                continue;
            }
            _ => {
                println!("Unknown command. Type 'h' for help.");
                continue;
            }
        };

        if let Some(cmd) = command {
            // Send command to server
            let now = Instant::now();
            if let Err(e) = stream.write_all(cmd.as_bytes()).await {
                eprintln!("Failed to send command: {}", e);
                break;
            }

            // Read response from server
            let mut buffer = vec![0; 1024];
            match stream.read(&mut buffer).await {
                Ok(0) => {
                    println!("Server closed the connection");
                    break;
                }
                Ok(n) => {
                    let latency = now.elapsed();
                    let response = String::from_utf8_lossy(&buffer[..n]);
                    println!(
                        "[{} micros] Server response: {}",
                        latency.as_micros(),
                        response.trim()
                    );
                }
                Err(e) => {
                    eprintln!("Failed to read response: {}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}

fn handle_set_command(input: &str) -> Option<String> {
    let parts: Vec<&str> = input.splitn(3, ' ').collect();

    if parts.len() < 3 {
        println!("Usage: s <key> <value>");
        println!("Example: s mykey myvalue");
        return None;
    }

    let key = parts[1];
    let value = parts[2];

    if key.is_empty() || value.is_empty() {
        println!("Key and value cannot be empty");
        return None;
    }

    Some(format!("SET {} {}\n", key, value))
}

fn handle_get_command(input: &str) -> Option<String> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();

    if parts.len() < 2 {
        println!("Usage: g <key>");
        println!("Example: g mykey");
        return None;
    }

    let key = parts[1];

    if key.is_empty() {
        println!("Key cannot be empty");
        return None;
    }

    Some(format!("GET {}\n", key))
}

fn handle_delete_command(input: &str) -> Option<String> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();

    if parts.len() < 2 {
        println!("Usage: d <key>");
        println!("Example: d mykey");
        return None;
    }

    let key = parts[1];

    if key.is_empty() {
        println!("Key cannot be empty");
        return None;
    }

    Some(format!("DELETE {}\n", key))
}

fn handle_list_command() -> Option<String> {
    Some("LIST\n".to_string())
}

fn show_help() {
    println!();
    println!("Available commands:");
    println!("  s <key> <value>  - SET a key to a value");
    println!("  g <key>          - GET the value of a key");
    println!("  d <key>          - DELETE a key");
    println!("  l                - LIST all keys");
    println!("  q                - QUIT the client");
    println!("  h                - Show this HELP message");
    println!();
    println!("Examples:");
    println!("  s user:1 john    - Set user:1 to 'john'");
    println!("  g user:1         - Get the value of user:1");
    println!("  d user:1         - Delete user:1");
    println!("  l                - List all keys in the database");
    println!();
}

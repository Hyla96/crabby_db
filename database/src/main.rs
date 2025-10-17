use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio::net::{TcpListener, TcpStream};

mod commands;
mod memory;
mod utils;

use commands::processor::process_command;

async fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    println!("Reading from a client");

    loop {
        match stream.read(&mut buffer).await {
            // Added .await
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(bytes_read) => {
                println!("Read {} bytes", bytes_read);
                let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received: {}", data.trim());
                let response = process_command(&buffer[..bytes_read]).await;
                stream.write_all(response.as_bytes()).await?; // Added .await
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?; // Added .await
    println!("Server listening on 127.0.0.1:8080");

    loop {
        // Changed from for loop to loop
        match listener.accept().await {
            // Use accept() instead of incoming()
            Ok((stream, addr)) => {
                println!("New client connected: {}", addr);
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Failed to accept client: {}", e),
        }
    }
}

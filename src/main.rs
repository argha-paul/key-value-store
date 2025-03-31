mod store;
mod errors;

use crate::store::KvStore;
use crate::errors::KvStoreError;
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::Arc;
use clap::Parser;
use std::io::Write;


#[derive(Parser, Debug)]
#[command(name = "RustKV", about = "A simple key-value store in Rust")]
struct Args {
    #[arg(short, long, default_value = "7878")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let store = Arc::new(KvStore::new());

    let addr = "127.0.0.1:7878";
    println!("Starting KV Store...");
    std::io::stdout().flush().unwrap();

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to port");

    println!("KV Store running on port {}", args.port);
    std::io::stdout().flush().unwrap();

    while let Ok((socket, _)) = listener.accept().await {
        std::io::stdout().flush().unwrap();
        let store = store.clone();
        std::io::stdout().flush().unwrap();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split(); 
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                println!("Received raw input: {}", line); 
                let response = handle_command(&store, line.trim());
                println!("Sending response: {}", response);
                if let Err(e) = writer.write_all(response.as_bytes()).await {
                    eprintln!("Failed to send response: {}", e);
                }
                if let Err(e) = writer.flush().await {
                    eprintln!("Failed to flush response: {}", e);
                }
                line.clear();
            }
        });
    }
}

fn handle_command(store: &KvStore, command: &str) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();
    println!("Received command: {}", command); 
    match parts.as_slice() {
        ["SET", key, value] => {
            store.set(key.to_string(), value.to_string());
            format!("OK: Key '{}' set to '{}'\n", key, value)
        }
        ["GET", key] => match store.get(key) {
            Some(value) => format!("{}\n", value),
            None => KvStoreError::KeyNotFound(key.to_string()).to_string(),
        },
        ["DELETE", key] => {
            if store.delete(key) {
                format!("OK: Key '{}' deleted\n", key)
            } else {
                KvStoreError::KeyNotFound(key.to_string()).to_string()
            }
        }
        _ => KvStoreError::InvalidInput.to_string(),
    }
}

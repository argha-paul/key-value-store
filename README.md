# High-Performance Key-Value Store (Rust)

A high-performance, in-memory key-value store built with Rust. This project demonstrates ownership, borrowing, lifetimes, memory safety, concurrency, multi-threading, async programming, and optimization techniques.

## Features
- **Fast & Efficient**: Uses `tokio` for async networking and concurrency.
- **Thread-Safe**: Utilizes `tokio::sync::RwLock` for concurrent access.
- **Command Support**:
  - `SET key value` - Stores a key-value pair.
  - `GET key` - Retrieves the value for a given key.
  - `DELETE key` - Removes the key from storage.
- **Testing**:
  - Unit tests for core functionalities.
  - Integration tests for networking and data persistence.
- **Optimized**: Implements property-based testing and performance tuning.

## Setup Instructions

### Prerequisites
- Install Rust and Cargo: [Install Rust](https://www.rust-lang.org/tools/install)
- Ensure `tokio` and `clap` dependencies are available.

### Installation
Clone the repository and navigate into the project directory:
```sh
# Clone the repository
git clone https://github.com/yourusername/rust-kv-store.git
cd rust-kv-store

# Build the project
cargo build --release
```

### Running the Server
Start the key-value store server by specifying a port:
```sh
cargo run -- --port 7878
```
Expected output:
```
KV Store running on port 7878
```

### Testing Connectivity
#### Using Netcat:
```sh
printf "SET name Rust\n" | nc 127.0.0.1 7878
```
```sh
printf "GET name\n" | nc 127.0.0.1 7878
```

#### Using Telnet (if available):
```sh
telnet 127.0.0.1 7878
SET name Rust
GET name
```

### Running Tests
To verify correctness, run the test suite:
```sh
cargo test
```

## Contributing
Feel free to fork this project and submit pull requests! Follow Rust best practices and add tests for new features.

## License
This project is open-source and available under the MIT License.


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
git clone https://github.com/argha-paul/key-value-store.git
cd key-value-store

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

**Command:**
```sh
printf "SET name Rust\n" | nc 127.0.0.1 7878
```
**Expected output:**
```
Key set successfully
```

**Command:**
```sh
printf "GET name\n" | nc 127.0.0.1 7878
```
**Expected output:**
```
Rust
```

**Command:**
```sh
printf "DELETE name\n" | nc 127.0.0.1 7878
```
**Expected output:**
```
Key deleted successfully
```

#### Using Telnet (if available):

**Command:**
```sh
telnet 127.0.0.1 7878
```
```
SET name Rust
```
**Expected output:**
```
Key set successfully
```

**Command:**
```
GET name
```
**Expected output:**
```
Rust
```

**Command:**
```
DELETE name
```
**Expected output:**
```
Key deleted successfully
```

### Running Tests
To verify correctness, run the test suite:
```sh
cargo test
```


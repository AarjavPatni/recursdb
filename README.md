# recursdb

recursdb is a simple Rust-based in-memory key-value store that operates over a TCP connection.

## Features

- **TCP-based interface:** Clients can connect to the server over TCP (default: `127.0.0.1:4000`).
- **In-memory storage:** All key-value pairs are stored in memory using a Rust `HashMap`.
- **Basic operations:**
  - `set`: Store a key-value pair.
  - `get`: Retrieve the value for a given key.

## Usage

1. **Build and run the server:**
   ```sh
   cargo run
   ```
   The server will start listening on `127.0.0.1:4000`.

2. **Connect using curl:**

   You can use `curl` to interact with the server.

   - To set a value:
     ```sh
     curl "http://127.0.0.1:4000/set=key=value"
     ```
   - To get a value:
     ```sh
     curl "http://127.0.0.1:4000/get=key"
     ```

   The server responds with the value for `get` requests.

## Limitations
- Data is **not persisted**; all data is lost when the server stops.
- Only one connection is handled at a time (no concurrency).
- No authentication or security features.

## Roadmap

- **Upcoming feature:** Store key-value pairs on the local file system for persistence.
- **Future possible improvements:**
  - Support for concurrent connections (multi-threading or async IO).
  - Additional commands and features.
  - Security and authentication.

## License

This project is for educational purposes and does not yet include a license file. 
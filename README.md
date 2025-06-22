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

## Design Decisions
- All data fits in memory; there is no limit on the number or size of key-value pairs (other than system memory).
- There is a limit on the size of input in the URL query: each request can be up to 1024 bytes (1 KB).
- Only one client connects at a time; concurrent connections are not supported.
- Keys and values are expected to be valid UTF-8 strings.
- The protocol expects requests in the form of HTTP GET lines (e.g., `/set?somekey=somevalue` or `/get?key=somekey`).
- No authentication or authorization is required; all clients are trusted.
- The server is always started on port 4000 at `127.0.0.1:4000`.

## Roadmap

- **Upcoming feature:** Store key-value pairs on the local file system for persistence.
- **Future possible improvements:**
  - Support for concurrent connections (multi-threading or async IO).
  - Additional commands and features.
  - Security and authentication.

## License

This project is for educational purposes and does not yet include a license file. 
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn parse_request(request: &String) -> (String, String) {
    let request_type: String = request[1..4].to_string();
    let rest = request[5..].to_string();

    (request_type, rest)
}

fn client_handler(mut stream: TcpStream, data: &mut HashMap<String, String>) -> Result<String, &str> {
    let mut buffer = [0; 1024];

    let request_size = stream.read(&mut buffer).map_err(|_| "Error saving to buffer")?;
    let mut request = String::from_utf8_lossy(&buffer[..request_size]).to_string();

    let line = request.lines().next().ok_or("Request line missing")?.to_string();
    request = line.split(' ').nth(1).ok_or("Request not found")?.to_string();
    let (request_type, rest) = parse_request(&request);

    match request_type.as_str() {
        "set" => {
            let parts: Vec<&str> = rest.split('=').collect();
            let key = parts[0].to_string();
            let value = parts[1].to_string();

            data.insert(key.clone(), value.clone());
            let response = format!("HTTP/1.1 200 OK\r\nConnection: close\r\n\r\nOK");
            let _ = stream.write_all(response.as_bytes());

            Ok(format!("set {}={}", &key, &value))
        }

        "get" => {
            let parts: Vec<&str> = rest.split('=').collect();
            let key = parts[1].to_string();
            let value = match data.get(&key) {
                Some(v) => {
                    v
                }

                None => {
                    "ERROR: Invalid key"
                }
            };

            let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", value.len(), value);
            let _ = stream.write_all(response.as_bytes());

            Ok(value.to_string())
        }

        _ => {
            Err("Invalid request")
        }
    }

    // println!("{request_type}");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    let mut data: HashMap<String, String> = HashMap::new();

    println!("Listening on localhost... 🦀 👀\n");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                match client_handler(stream, &mut data) {
                    Ok(result) => {
                        println!("{}", result.to_string());
                    }

                    Err(e) => {
                        println!("{}", e.to_string());
                    }
                }
                println!("Connection closed\n");
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(())
}

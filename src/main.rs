use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn client_handler(mut stream: TcpStream, data: &mut HashMap<String, String>) {
    let mut request: String = String::new();

    stream.read_to_string(&mut request).unwrap().to_string();

    request = request.lines().next().unwrap().split(' ').nth(1).unwrap().to_string();
    let request_type: String = request[1..4].to_string();

    match request_type.as_str() {
        "set" => {
            let rest = request[5..].to_string();
            let parts: Vec<&str> = rest.split('=').collect();
            let key = parts[0].to_string();
            let value = parts[1].to_string();
            println!("{} - {}", &key, &value);
            data.insert(key, value);
        }

        "get" => {
            todo!()
        }

        _ => {
            println!("Invalid request")
        }
    }

    // println!("{request_type}");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    let mut data: HashMap<String, String> = HashMap::new();

    println!("Listening on localhost... 🦀 👀");

    // TODO: Connections persist unless forced to terminate
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                client_handler(stream, &mut data);
                println!("Connection closed");
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(())
}

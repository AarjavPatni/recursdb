use std::net::{TcpListener, TcpStream};
use std::io::{Read};

fn client_handler(mut stream: TcpStream) {
    let mut request: String = String::new();

    stream.read_to_string(&mut request).unwrap().to_string();

    request = request.lines().next().unwrap().to_string();
    let request_type: String = request.split(' ').nth(1).unwrap()[1..4].to_string();

    match request_type.as_str() {
        "set" => {
            todo!()
        }

        "get" => {
            todo!()
        }

        _ => {
            println!("Invalid request")
        }
    }

    println!("{request_type}");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    println!("Listening on localhost... 🦀 👀");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                client_handler(stream);
                println!("Connection closed");
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(())
}

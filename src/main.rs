use std::net::{TcpListener, TcpStream};
use std::io::{Read};

fn client_handler(mut stream: TcpStream) {
    let mut input: String = String::new();

    stream.read_to_string(&mut input).unwrap().to_string();
    println!("{input}");
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

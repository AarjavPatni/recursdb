use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    println!("listening on localhost... 🦀 👀");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(())
}

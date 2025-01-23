use std::io;
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    todo!();
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}

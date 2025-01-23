use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("home.html")?;
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

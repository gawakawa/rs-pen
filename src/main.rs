use std::io::prelude::*;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let markdown = "Hello";
    let html = markdown;

    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{html}");
        stream.write_all(response.as_bytes())?;
    }
    Ok(())
}

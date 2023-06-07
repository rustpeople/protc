use std::net::{TcpStream, TcpListener};
use std::io::Result as IoResult;
use core::fmt::Display;

pub fn server<D: Display, F: Fn(&mut TcpStream) -> IoResult<()>>(host: D, port: u16, handle: F) -> IoResult<()> {
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        handle(&mut stream)?;
    }

    Ok(())
}

pub fn client<D: Display, F: Fn(&mut TcpStream) -> IoResult<()>>(host: D, port: u16, handle: F) -> IoResult<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;
    handle(&mut stream)?;
    Ok(())
}

// I think we've finished tcp
// We'll implement http (and all versions)
// We need http, ftp, ...
// You're right
// See src/http/v1_1.rs
use std::net::{TcpStream, TcpListener};

use std::io::Result as IoResult;
use core::fmt::Display;

pub fn server<D: Display, F: Fn(&mut TcpStream) -> IoResult<()>>(host: D, port: u16, handle: F) -> IoResult<()> {
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;

    loop {
        let (mut stream, _) = listener.accept()?;

        handle(&mut stream)?;
    }
}

pub fn client<D: Display, F: Fn(&mut TcpStream) -> IoResult<()>>(host: D, port: u16, handle: F) -> IoResult<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;
    handle(&mut stream)?;
    Ok(())
}
#[cfg(target_os = "linux")]
use std::os::unix::net::{UnixStream as TcpStream, UnixListener as TcpListener};

#[cfg(not(target_os = "linux"))]
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
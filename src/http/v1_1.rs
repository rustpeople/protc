use std::collections::HashMap;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Invalid request")]
    InvalidRequest,
    #[error("Invalid header format")]
    InvalidHeaderFormat,
    #[error("Invalid status")]
    InvalidStatus,
}

#[derive(Debug, Default, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub fields: HashMap<String, String>,
    pub body: String
}

impl Request {
    pub fn parse(request_line: String) -> Result<Request, Error> {
        // Split the request into lines
        let lines: Vec<String> = request_line.lines().map(|s| s.to_string()).collect();

        // Extract the method and path from the first line
        let first_line_parts: Vec<String> = lines[0].split_whitespace().map(|s| s.to_string()).collect();
        if first_line_parts.len() != 3 {
            return Err(Error::InvalidRequest);
        }
        let method = first_line_parts[0].to_string();
        let path = first_line_parts[1].to_string();

        // Parse the headers and body
        let mut fields = HashMap::new();
        let mut body = String::new();
        let mut parsing_body = false;

        for line in lines.iter().skip(1) {
            if parsing_body {
                // Add the line to the body
                body.push_str(line);
                body.push('\n');
            } else if line.is_empty() {
                // Empty line indicates end of headers
                parsing_body = true;
            } else {
                // Parse headers
                let header_parts: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
                if header_parts.len() != 2 {
                    return Err(Error::InvalidHeaderFormat);
                }
                let key = header_parts[0].to_string();
                let value = header_parts[1].to_string();
                fields.insert(key, value);
            }
        }

        Ok(Request {
            method,
            path,
            fields,
            body,
        })
    }
}

impl core::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from(format!("{} {} HTTP/1.1\r\n", self.method, self.path));
        for i in self.fields.keys() {
            result += format!("{}: {}\r\n", i, self.fields.get(i).unwrap()).as_str();
        }
        result += format!("\r\n{}", self.body).as_str();
        write!(f, "{}", result)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Response {
    pub status: u16,
    pub fields: HashMap<String, String>,
    pub body: String
}

impl Response {
    pub fn parse(data: String) -> Result<Self, Error> { // No need to parse the response
        // Split the request into lines
        let lines: Vec<&str> = data.lines().collect();

        // Extract the method and path from the first line
        let first_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
        if first_line_parts.len() != 2 && first_line_parts.len() != 3  {
            return Err(Error::InvalidRequest);
        }

        let status: u16 = match first_line_parts[1].to_string().as_str().parse() {
            Ok(v) => v,
            Err(_e) => return Err(Error::InvalidStatus)
        };

        let mut fields = HashMap::new();
        let mut body = String::new();
        let mut parsing_body = false;

        for line in lines.iter().skip(1) {
            if parsing_body {
                // Add the line to the body
                body.push_str(line);
                body.push_str("\n");
            } else if line.is_empty() {
                // Empty line indicates end of headers
                parsing_body = true;
            } else {
                // Parse headers
                let header_parts: Vec<String> = line.splitn(2, ": ").map(|s| s.to_string()).collect();
                if header_parts.len() != 2 {
                    return Err(Error::InvalidHeaderFormat);
                }
                let key = header_parts[0].to_string();
                let value = header_parts[1].to_string();
                fields.insert(key, value);
            }
        }

        Ok(Self { status: status, fields: fields, body: body })
    }
}

impl core::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from(format!(
            "HTTP/1.1 {}\r\n{:#?}\r\n\r\n{}",
            self.status,
            self.fields.iter().map(|(k, v)| format!("{k}:{v}")).collect::<Vec<String>>(),
            self.body,
        ));
        for i in self.fields.keys() {
            result += format!("{}: {}\r\n", i, self.fields.get(i).unwrap()).as_str()
        }
        result += format!("\r\n{}", self.body).as_str();
        write!(f, "{}", result)
    }
}

// same as tcp, but handle takes a Request and returns a Response
#[cfg(target_os = "linux")]
use std::os::unix::net::{UnixStream as TcpStream, UnixListener as TcpListener};

#[cfg(not(target_os = "linux"))]
use std::net::{TcpStream, TcpListener};
use std::io::{Result as IoResult, Read, Write};
use core::fmt::Display;

pub fn server<D: Display, F: Fn(Request) -> Response>(host: D, port: u16, handle: F) -> IoResult<()> {
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;

    loop {
        let (mut stream, _) = listener.accept()?;
        let mut data = String::new();
        stream.read_to_string(&mut data)?;
        let request = Request::parse(data).unwrap();
        let response = handle(request);
        stream.write_all(response.to_string().as_bytes())?;
        stream.flush()?;
    }
}

pub fn client<D: Display>(host: D, port: u16, request: Request) -> IoResult<Response> {
    let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;
    stream.write_all(request.to_string().as_bytes())?;
    let mut data = String::new();
    stream.read_to_string(&mut data)?;
    Ok(Response::parse(data).unwrap())
}
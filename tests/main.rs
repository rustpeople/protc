extern crate protc;

use std::collections::HashMap;

use std::thread;

use protc::http::v1_1::{server, client, Request, Response};

fn testing_server(req: Request) -> Response {
    if req.method == "GET" && req.path == "/" {
        Response {
            status: 200,
            fields: HashMap::new(),
            body: "Hello, world!".into()
        }
    } else {
        panic!("wrong mate")
    }
}
fn testing_client() -> Request {
    Request { method: "GET".into(), path: "/".into(), fields: HashMap::new(), body: "Hello, world!".into() }
}

const THREAD_FAIL_MSG: &str = "Failed to execute thread";

#[test]
fn main() { // Now run `cargo test` in the terminal but can you share the terminal? Yes
    let t1 = thread::spawn(||
        server("127.0.0.1", 12000, testing_server).expect("Server failed to start")
    );
    t1.join().expect(THREAD_FAIL_MSG);
    eprintln!("CLIENT RESULT: {}", client("127.0.0.1", 12000, testing_client()).expect("Client failed to start"));
}

// on last version (before you joined), the server only wrote to a stream when it was closed, even if the code was correct
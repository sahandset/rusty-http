use std::{
    env,
    io::{Read, Write},
    net::TcpListener,
};

use rusty_http::http::HttpRequest;

const ADDR: &str = "127.0.0.1";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Error starting server -- please specify a port #!")
    }
    let port: &str = &args[1];
    let listener = TcpListener::bind(format!("{ADDR}:{port}")).unwrap();
    println!("Listening on port {}...", port);
    for stream in listener.incoming() {
        let mut buffer: [u8; 1024] = [0; 1024];
        match stream {
            Ok(mut stream) => {
                let n = stream.read(&mut buffer);
                match n {
                    Ok(_n) => {
                        let req = HttpRequest::handle_request(&mut buffer);
                        println!("{:#?}", req);
                        let response: &str = "HTTP/1.1 200 OK\r\n\r\nHello\n";
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                    Err(_) => {
                        panic!("Error reading in to buffer")
                    }
                }
            }
            Err(_) => {
                panic!("Error creating a stream listener");
            }
        }
    }
}

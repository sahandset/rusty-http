use std::{
    env,
    io::{Read, Write},
    net::TcpStream,
};

use rusty_http::http::HttpResponse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("Usage: client <host> <port> <path> [method] [body]");
    }

    let host = &args[1];
    let port = &args[2];
    let path = &args[3];
    let method = args.get(4).map(|s| s.as_str()).unwrap_or("GET");
    let body = args.get(5).cloned().unwrap_or_default();

    let addr = format!("{host}:{port}");
    let mut stream = TcpStream::connect(&addr).unwrap();

    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {addr}\r\nContent-Length: {}\r\nAccept: */*\r\n\r\n{body}",
        body.len()
    );

    stream.write_all(request.as_bytes()).unwrap();

    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).unwrap();

    let response = HttpResponse::from_bytes(&buffer[..n]);
    println!("{:#?}", response);
}

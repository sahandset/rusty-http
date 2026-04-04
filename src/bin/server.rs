use std::{
    env,
    io::{Read, Write},
    net::TcpListener,
};

use rusty_http::http::{HttpRequest, HttpResponse, Router, request::Method};

const ADDR: &str = "127.0.0.1";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Error starting server -- please specify a port #!")
    }
    let port: &str = &args[1];
    let listener = TcpListener::bind(format!("{ADDR}:{port}")).unwrap();
    let router = Router::new();
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
                        let res = router.match_route(req);
                        stream.write_all(&res.to_bytes()).unwrap();
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

fn register_routes() -> Router {
    let router = Router::new();
    router.add_route(Method::GET, "/", home_handler);
    router.add_route(Method::GET, "/about", about_handler);
    router.add_route(Method::GET, "/echo", echo_handler);
}

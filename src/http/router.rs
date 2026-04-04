use std::collections::HashMap;

use crate::http::{HttpRequest, HttpResponse, request::Method};

pub type Handler = fn(&HttpRequest) -> HttpResponse;

pub struct Router {
    routes: Vec<(Method, String, Handler)>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn add_route(&mut self, method: Method, path: String, handler: Handler) {
        self.routes.push((method, path, handler));
    }

    pub fn match_route(&self, request: HttpRequest) -> HttpResponse {
        match request.path.as_str() {
            "/" => {
                let body = String::from("Welcome to rusty-http!");
                HttpResponse::new(
                    200,
                    String::from("OK"),
                    HashMap::from([("Content-Length".to_string(), body.len().to_string())]),
                    body,
                )
            }
            "/about" => {
                let body = String::from("A lightweight HTTP Server...");
                HttpResponse::new(
                    200,
                    String::from("OK"),
                    HashMap::from([("Content-Length".to_string(), body.len().to_string())]),
                    body,
                )
            }
            "/echo" => {
                let body = String::from(request.body);
                HttpResponse::new(
                    200,
                    String::from("OK"),
                    HashMap::from([("Content-Length".to_string(), body.len().to_string())]),
                    body,
                )
            }
            _ => {
                let body = String::from("That endpoint does not exist!");
                HttpResponse::new(
                    404,
                    String::from("Not Found"),
                    HashMap::from([("Content-Length".to_string(), body.len().to_string())]),
                    body,
                )
            }
        }
    }
}

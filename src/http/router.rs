use std::collections::HashMap;

use crate::http::{HttpRequest, HttpResponse, request::Method};

pub type Handler = fn(request: &HttpRequest) -> HttpResponse;
pub type Route = (Method, String, Handler);
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn add_route(&mut self, method: Method, path: String, handler: Handler) {
        self.routes.push((method, path, handler));
    }

    pub fn match_route(&self, request: &HttpRequest) -> HttpResponse {
        let req_route = self
            .routes
            .iter()
            .find(|(method, path, _)| *method == request.method && *path == request.path);
        match &req_route {
            Some((_, _, handler)) => handler(request),
            None => HttpResponse::new(
                404,
                String::from("Not Found"),
                HashMap::new(),
                String::from("404 Not Found"),
            ),
        }
    }
}

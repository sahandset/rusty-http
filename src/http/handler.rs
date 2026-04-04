use std::collections::HashMap;

use crate::http::{HttpRequest, HttpResponse};

pub fn home_handler(_request: &HttpRequest) -> HttpResponse {
    let response_body = String::from("Welcome to rust-http!\n");
    HttpResponse::new(
        200,
        String::from("Ok"),
        HashMap::from([(
            "Content-Length".to_string(),
            response_body.len().to_string(),
        )]),
        response_body,
    )
}

pub fn about_handler(_request: &HttpRequest) -> HttpResponse {
    let response_body = String::from("A lightweight HTTP server written in Rust\n");
    HttpResponse::new(
        200,
        String::from("Ok"),
        HashMap::from([(
            "Content-Length".to_string(),
            response_body.len().to_string(),
        )]),
        response_body,
    )
}

pub fn echo_handler(request: &HttpRequest) -> HttpResponse {
    let response_body = format!("{}{}", request.body, "\n");
    HttpResponse::new(
        200,
        String::from("Ok"),
        HashMap::from([(
            "Content-Length".to_string(),
            response_body.len().to_string(),
        )]),
        response_body,
    )
}

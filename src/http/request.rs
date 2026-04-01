use std::collections::HashMap;

pub enum Method {
    POST,
    GET,
    PUT,
    DELETE,
    UNKNOWN,
}

pub struct HttpRequest {
    pub method: Method,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

// Need to clean this up -- curently parsing & splitting request from buffer incorrectly

impl HttpRequest {
    pub fn handle_request(buffer: &[u8]) -> HttpRequest {
        let req_string = String::from_utf8_lossy(buffer);
        let req_components_by_line: Vec<&str> = req_string.split("\r\n").collect();
        let method_path_version = req_components_by_line[0]
            .splitn(3, ' ')
            .collect::<Vec<&str>>();
        HttpRequest {
            method: match method_path_version[0] {
                "POST" => Method::POST,
                "GET" => Method::GET,
                "PUT" => Method::PUT,
                "DELETE" => Method::DELETE,
                _ => Method::UNKNOWN,
            },
            path: method_path_version[1].to_string(),
            http_version: method_path_version[2].to_string(),
            headers: Self::parse_headers(&req_components_by_line[3]),
            body: req_components_by_line[4].to_string(),
        }
    }
    fn parse_headers(header_string: &str) -> HashMap<String, String> {
        header_string
            .lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let mut split = line.splitn(2, ':');
                let key = split.next()?.trim();
                let val = split.next()?.trim();
                Some((key.to_string(), val.to_string()))
            })
            .collect()
    }
}

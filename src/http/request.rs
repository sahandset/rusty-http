use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    POST,
    GET,
    PUT,
    DELETE,
    UNKNOWN,
}
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn handle_request(buffer: &[u8]) -> HttpRequest {
        let req_string = String::from_utf8_lossy(buffer);
        let req_components_by_line: Vec<&str> = req_string.split("\r\n").collect();
        let method_path_version = req_components_by_line[0]
            .splitn(3, ' ')
            .collect::<Vec<&str>>();
        let headers_ending_idx = req_components_by_line
            .iter()
            .position(|line| line.is_empty())
            .unwrap();
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
            headers: Self::parse_headers(&req_components_by_line[1..headers_ending_idx]),
            body: req_components_by_line[headers_ending_idx + 1..]
                .join("\r\n")
                .trim_matches('\0')
                .to_string(),
        }
    }
    fn parse_headers(header_string: &[&str]) -> HashMap<String, String> {
        header_string
            .iter()
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

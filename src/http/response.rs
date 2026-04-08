use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(
        status_code: u16,
        status_text: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> HttpResponse {
        HttpResponse {
            status_code: status_code,
            status_text: status_text,
            headers: headers,
            body: body,
        }
    }

    pub fn from_bytes(buffer: &[u8]) -> HttpResponse {
        let raw = String::from_utf8_lossy(buffer);
        let parts: Vec<&str> = raw.splitn(2, "\r\n\r\n").collect();

        let head = parts.get(0).unwrap_or(&"");
        let body = parts.get(1).unwrap_or(&"").trim_matches('\0').to_string();

        let mut head_lines = head.lines();
        let status_line = head_lines.next().unwrap_or("HTTP/1.1 200 OK");
        let mut status_parts = status_line.splitn(3, ' ');
        let _version = status_parts.next().unwrap_or("HTTP/1.1");
        let status_code: u16 = status_parts.next().unwrap_or("200").parse().unwrap_or(200);
        let status_text = status_parts.next().unwrap_or("OK").to_string();

        let headers = head_lines
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let mut split = line.splitn(2, ": ");
                let key = split.next()?.trim().to_string();
                let val = split.next()?.trim().to_string();
                Some((key, val))
            })
            .collect();

        HttpResponse { status_code, status_text, headers, body }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let header_string: String = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join("\r\n");

        format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status_code, self.status_text, header_string, self.body
        )
        .into_bytes()
    }
}

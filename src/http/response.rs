use std::collections::HashMap;

pub struct HttpResponse {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
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

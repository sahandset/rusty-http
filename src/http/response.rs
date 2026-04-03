use std::collections::HashMap;

pub struct HttpResponse {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpResponse {
    pub fn handle_response() -> HttpResponse {
        let body = String::from("Hello");
        let headers = HashMap::from([("Content-Length".to_string(), body.len().to_string())]);
        HttpResponse {
            status_code: 200,
            status_text: "Ok".to_string(),
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

use std::collections::HashMap;

/// Parsed reprsentation of an HTTP message.
#[derive(Debug)]
pub struct HTTPMessage {
    pub start_line: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HTTPMessage {
    /// Parse a raw HTTP Message string into a struct.
    pub fn new(raw_message: &str) -> HTTPMessage {
        let (headers, body) = raw_message.split_once("\r\n\r\n").unwrap();
        let lines = headers.lines().enumerate();
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut start_line = String::new();

        for (idx, line) in lines {
            if idx == 0 {
                start_line = line.to_string();
                continue;
            }
            let (key, value) = line.split_once(": ").unwrap();
            headers.insert(key.to_string(), value.to_string());
        }

        let body = body.to_string().into_bytes();

        HTTPMessage {
            start_line,
            headers,
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HTTPMessage;

    #[test]
    fn parse_raw_message() {
        let http_message =
            "HTTP/1.1 200 OK\r\nAccept-Ranges: bytes\r\nContent-Length: 100\r\n\r\nSome content.\nAnd some more.";

        let message = HTTPMessage::new(http_message);
        let message_str = format!("{:?}", message);
        let expected_message_str = r#"HTTPMessage { start_line: "HTTP/1.1 200 OK", headers: {"Content-Length": "100", "Accept-Ranges": "bytes"}, body: [83, 111, 109, 101, 32, 99, 111, 110, 116, 101, 110, 116, 46, 10, 65, 110, 100, 32, 115, 111, 109, 101, 32, 109, 111, 114, 101, 46] }"#;

        assert_eq!(message_str, expected_message_str);
    }
}

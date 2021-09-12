use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

use crate::{find_subsequence, HTTPMessage};

/// Size of the buffer used to store the incoming stream.
static BUFFER_SIZE: usize = 409600;

/// Perform an HTTP HEAD request to get metadata without the body.
pub fn head(address: &str, path: &str) -> HTTPMessage {
    let mut stream = TcpStream::connect(address).expect("Unable to connect to server");
    let (host, _) = address.split_once(":").unwrap();
    let request = format!(
        "HEAD {} HTTP/1.1\r\nHost: {}\r\nAccept: */*\r\n\r\n",
        path, host
    );
    stream.write(request.as_bytes()).unwrap();
    let mut buffer: Vec<u8> = vec![0; 1024];
    stream.read(&mut buffer).unwrap();

    HTTPMessage::new(str::from_utf8(&buffer).unwrap())
}

/// Partially download a file in the given byte range.
pub fn download_part(address: String, path: String, bytes_from: usize, bytes_to: usize) -> Vec<u8> {
    let mut stream = TcpStream::connect(&address).expect("Unable to connect to server");
    let (host, _) = address.split_once(":").unwrap();
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nRange: bytes={}-{}\r\nAccept: */*\r\n\r\n",
        path, host, bytes_from, bytes_to
    );
    stream.write(request.as_bytes()).unwrap();

    let body_size = bytes_to - bytes_from + 1;
    let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];
    let mut body: Vec<u8> = Vec::with_capacity(body_size);
    let body_break: Vec<u8> = "\r\n\r\n".as_bytes().to_vec();
    let mut content_consumed: usize = 0;

    let mut first = true;

    // Keep reading until all data is received.
    while content_consumed < body_size {
        let bytes_read = stream.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        // If reading the stream for the first time, parse the headers, otherwise just append
        // the buffer to body.
        if first {
            first = false;
            let loc_body_break = find_subsequence(&buffer, &body_break).unwrap();
            let metadata = str::from_utf8(&buffer[0..loc_body_break + 4]).unwrap();
            let message = HTTPMessage::new(metadata);
            let _content_length: usize =
                str::parse(message.headers.get("Content-Length").unwrap()).unwrap();
            let content = &buffer[loc_body_break + 4..bytes_read];
            content_consumed += content.len();
            body.extend_from_slice(content);
        } else {
            content_consumed += bytes_read;
            body.extend_from_slice(&buffer[0..bytes_read]);
        }
    }

    assert_eq!(body.len(), body_size);

    return body;
}

use std::{
    fmt::format,
    io::{Read, Write},
    net::TcpListener,
};

use std::net::{SocketAddr, ToSocketAddrs};

use std::str;

use crate::{find_subsequence, HTTPMessage};

pub fn download_part(
    address: &str,
    path: &str,
    bytes_from: usize,
    bytes_to: usize,
) -> Option<Vec<u8>> {
    let addr = address.to_socket_addrs().unwrap().next().unwrap();
    println!("{:?}", addr);
    let listener = TcpListener::bind(addr).unwrap();
    let (host, _) = address.split_once(":").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let request = format!(
                    "GET {} HTTP/1.1\r\nHost: {}\r\nRange: bytes={}-{}\r\nAccept: */*\r\n\r\n",
                    path, host, bytes_from, bytes_to
                );
                stream.write(request.as_bytes());

                let body_size = bytes_to - bytes_from + 1;
                let mut buffer: Vec<u8> = Vec::with_capacity(body_size + 1024);
                let body_break: Vec<u8> = vec![13, 10];
                stream.read_to_end(&mut buffer).unwrap();

                let loc_body_break = find_subsequence(&buffer, &body_break).unwrap();
                let metadata = str::from_utf8(&buffer[0..loc_body_break + 2]).unwrap();
                let message = HTTPMessage::new(metadata);
                println!("{:?}", message);
            }
            Err(e) => {
                eprintln!("Failed to get stream from Tcp Listener : {}", e);
            }
        }

        return None;
    }

    unimplemented!()
}

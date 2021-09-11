use multidl::{find_subsequence, tcp::download_part};

use std::net::{TcpListener, TcpStream, ToSocketAddrs};

fn main() {
    let server_details = "example.com:80";
    let server: Vec<_> = server_details
        .to_socket_addrs()
        .expect("Unable to resolve domain")
        .collect();
    println!("{:?}", server);

    // Even easier, if you want to connect right away:
    let stream = TcpStream::connect(server_details).expect("Unable to connect to server");
    // let listener = TcpListener::bind(server[1]).unwrap();
}

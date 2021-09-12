use multidl::download;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let server_details = &args[1];
    let path = &args[2];

    download(server_details, path)
}

use multidl::download;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || &args[1].to_lowercase() == "--help" {
        print_usage(&args[0]);
        exit(1);
    }

    let server_details = &args[1];
    let path = &args[2];

    println!("Starting file downloading. Requesting content-length from server...");
    // Start the file download. The file is stored in `data.bin`.
    download(server_details, path);
}

/// Print command usage instructions.
fn print_usage(cmd: &str) {
    println!("Usage: {} [--help] ADDRESS_WITH_PORT PATH_TO_FILE", cmd);
    println!("Download a file in multiple parts in parallel into data.bin.\n");
    println!("--help - Show this help message, and exit with code 1.");
    println!("ADDRESS_WITH_PORT - Can be IP address or a domain name. ( example 127.0.0.1:3000 or cdn.example.com:80 )");
    println!("PATH_TO_FILE - Absoulte path to the file, including preceding slash. ( example / or /public/file.bin )");
}

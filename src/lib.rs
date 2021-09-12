//! Download a file in multiple parts in parallel.
//!
//! Makes uses of HTTP Partial Content ( HTTP 206 ) and sends byte-range headers.
//! Downloads parts of the file in parallel and merges them in the end. This can
//! increase download speeds when individual connections are throttled.

mod http_message;
pub mod tcp;
use std::fs;
use std::path::PathBuf;
use std::{cmp::min, str, thread};

pub use crate::http_message::HTTPMessage;
use crate::tcp::{download_part, head};

/// Maximum number of bytes that can be downloaded by each thread, i.e.,
/// max size of divisions of the target file.
static DIVISIONS: usize = 409600;

/// Find the fist instance of `needle` in `haystack`.
pub fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where
    for<'a> &'a [T]: PartialEq,
{
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// Download the file in the given server and path.
pub fn download(server_details: &str, path: &str) {
    let head_req = head(&server_details, path);

    // Only proceed if byte-range downloads are supported by the server.
    if let Some(value) = head_req.headers.get("Accept-Ranges") {
        if value != "bytes" {
            eprintln!("Server does not support downloading byte ranges");
            return;
        }

        let content_length: usize =
            str::parse(head_req.headers.get("Content-Length").unwrap()).unwrap();

        println!("Total Content-Length: {}", content_length);

        let mut buf: Vec<u8> = Vec::with_capacity(content_length);
        let mut threads = Vec::new();

        let mut start_at = 0;
        let mut i = 0;

        // Keep spawning threads until all bytes are covered.
        while start_at < content_length {
            i += 1;
            let address = server_details.to_string();
            let path = path.to_string();
            let end_at = min(start_at + DIVISIONS, content_length - 1);
            println!(
                "Downloading {} to {} bytes in thread {}",
                start_at, end_at, i
            );

            threads.push(thread::spawn(move || {
                let data = download_part(address, path, start_at, end_at);
                println!(
                    "Thread {} finished partial download of {} bytes.",
                    i,
                    data.len()
                );
                data
            }));

            start_at = end_at + 1;
        }

        for handle in threads {
            let data = handle.join().unwrap();
            buf.extend(data);
        }

        assert_eq!(buf.len(), content_length);

        let srcdir = PathBuf::from("./data.bin");

        fs::write(&srcdir, buf).unwrap();
        println!(
            "Finished merging parts. File stored in {:?}.",
            fs::canonicalize(&srcdir).unwrap()
        );
    } else {
        eprintln!("Mult-threaded downloads not supported for this request.");
        return;
    }
}

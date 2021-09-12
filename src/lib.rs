mod http_message;
pub mod tcp;
use std::fs;
use std::{cmp::min, str, thread, time};

pub use crate::http_message::HTTPMessage;
use crate::tcp::{download_part, head};

static DIVISIONS: usize = 302400;

pub fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where
    for<'a> &'a [T]: PartialEq,
{
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

pub fn download(server_details: &str, path: &str) {
    let head_req = head(&server_details, path);
    // println!("{:?}", head_req);

    if let Some(value) = head_req.headers.get("Accept-Ranges") {
        if value != "bytes" {
            eprintln!("Server does not support downloading byte ranges");
            return;
        }

        let content_length: usize =
            str::parse(head_req.headers.get("Content-Length").unwrap()).unwrap();

        println!("Content-Length: {}", content_length);

        let mut buf: Vec<u8> = Vec::with_capacity(content_length);
        let mut threads = Vec::new();

        let mut consumed = 0;
        let mut i = 0;
        while consumed < content_length {
            println!("{}", i);
            i += 1;
            let address = server_details.to_string();
            let path = path.to_string();

            threads.push(thread::spawn(move || {
                let data = download_part(
                    address,
                    path,
                    consumed,
                    min(consumed + DIVISIONS, content_length - 1),
                );
                data
            }));

            // let one_second = time::Duration::from_secs(1);
            // thread::sleep(one_second);

            consumed += DIVISIONS;
        }

        for handle in threads {
            let data = handle.join().unwrap();
            buf.extend(data);
        }

        fs::write("data.txt", buf).unwrap();
    } else {
        eprintln!("Mult-threaded downloads not supported for this request.");
        return;
    }
}

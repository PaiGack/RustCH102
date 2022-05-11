use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub mod mythread;

use crate::mythread::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    Read::read(&mut stream, &mut buffer).unwrap();
    println!("request:\n{}", String::from_utf8_lossy(&buffer));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        stream
            .write("HTTP/1.1 200 OK\r\nContent-Length: 5 \r\n\r\nHELLO \r\n\r\n".as_bytes())
            .unwrap();
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        stream
            .write("HTTP/1.1 200 OK\r\nContent-Length: 5 \r\n\r\nHELLO \r\n\r\n".as_bytes())
            .unwrap();
    } else {
        stream
            .write("HTTP/1.1 404 Not Found\r\nContent-Length: 13 \r\n\r\n404 Not Found \r\n\r\n".as_bytes())
            .unwrap();
    };
}

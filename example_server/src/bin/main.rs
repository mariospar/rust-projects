use example_server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let get = b"GET /hello HTTP/1.1\r\n";
        let (status, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contents = fs::read_to_string(filename).unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

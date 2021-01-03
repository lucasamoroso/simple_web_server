use simple_web_server::ThreadPool;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Couldn't bind server");
    let pool = ThreadPool::new(2);

    for stream in listener.incoming() {
        let stream_value = stream?;

        pool.execute(|| {
            handle_connection(stream_value)
                .map_err(|e| println!("Ups!! Something went wrong {}", e));
        })
    }

    println!("Shutting down.");
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer)?;

    let (status_line, filename) = if buffer.starts_with(b"GET") {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "src/html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/html/404.html")
    };

    let contents = fs::read_to_string(filename)?;

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

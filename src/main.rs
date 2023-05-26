use std::io::prelude::*;
use std::error::Error;
use std::net::{TcpStream, TcpListener};

mod thread_pool;
use thread_pool::ThreadPool;

mod http;
use http::{HTTPResponse, HTTPRequest};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            if handle_request(&stream).is_err() {
                handle_failed_request(&stream);
            };
        });
    }
}

/// Handle incoming requests
/// 
/// # Errors
/// 
/// Errors if handling request or sending response fails.
fn handle_request(mut stream: &TcpStream) -> Result<(), Box<dyn Error>>{
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request = HTTPRequest::new(buffer);
    let response = HTTPResponse::new(request)?;

    stream.write(response.text()?.as_bytes())?;
    stream.flush()?;

    Ok(())
}

/// Send error if request handling failed
fn handle_failed_request(mut stream: &TcpStream) {
    let response = HTTPResponse::from_error();
    let response_text = response.text().unwrap();
    stream.write(response_text.as_bytes()).unwrap();
    stream.flush().unwrap();
}

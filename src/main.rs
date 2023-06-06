use std::io::prelude::*;
use std::error::Error;
use std::fs;
use std::net::{TcpStream, TcpListener};

mod thread_pool;
use thread_pool::ThreadPool;

use http_rs::request::Request;
use http_rs::response::{Response, Status};

/// Handle incoming requests
/// 
/// # Errors
/// 
/// Errors if handling request or sending response fails.
fn handle_request(mut stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    /* parse request */
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    let message = String::from_utf8(buffer.to_vec()).unwrap();
    let request = Request::from_string(message);

    /* create response */
    let html_path = handle_routing(request.path.unwrap());
    let (content, status) = match fs::read_to_string(html_path.clone()) {
        Ok(val) => (val, Status::Ok),
        Err(_) => (fs::read_to_string("html/404.html")?, Status::NotFound),
    };
    let response = Response::new(Some(content), status);

    /* send response */
    stream.write(response.to_string().as_bytes())?;
    stream.flush()?;

    Ok(())
}

/// Get file path to html
fn handle_routing(request_path: String) -> String {
    let mut path = String::from("html");

    if request_path == "/" {
        path.push_str("/index.html");
    } else {
        path.push_str(&request_path);
        path.push_str(".html");
    }

    path
}

/// Send error if request handling failed
fn handle_failed_request(mut stream: &TcpStream) {
    let response = Response::new(None, Status::NotFound);
    stream.write(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            if handle_request(&stream).is_err() {
                println!("ignoring error");
                handle_failed_request(&stream);
            };
        });
    }
}

use std::fs;
use std::error::Error;

mod request;
use request::{Protocol, Version};

mod response;
use response::Status;

/// HTTP Request
#[allow(dead_code)]
#[derive(Debug)]
pub struct Request {
    protocol: Protocol,
    path: Option<String>,
    version: Version,
    host: Option<String>,
    user_agent: Option<String>,
    accept: Option<String>,
    accept_language: Option<String>,
    accept_encoding: Option<String>,
    dnt: Option<String>,
    connection: Option<String>,
    upgrade_insecure_requests: Option<String>,
    sec_fetch_dest: Option<String>,
    sec_fetch_mode: Option<String>,
    sec_fetch_site: Option<String>,
    sec_fetch_user: Option<String>,
}

impl Request {
    /// Create new HTTP Request from buffer
    pub fn from_buffer(buffer: [u8; 1024]) -> Self {
        let buffer = String::from_utf8(buffer.to_vec()).unwrap();
        let mut buffer_iter = buffer.split("\r\n");

        let first_line = buffer_iter.next().unwrap();
        let mut first_line_split = first_line.split(" ");
        let protocol = Protocol::from_str(first_line_split.next().unwrap_or_default());
        let path = Some(first_line_split.next().unwrap_or_default().to_string());
        let version = Version::from_str(first_line_split.next().unwrap_or_default());

        let mut host = None;
        let mut user_agent = None;
        let mut accept = None;
        let mut accept_language = None;
        let mut accept_encoding = None;
        let mut dnt = None;
        let mut connection = None;
        let mut upgrade_insecure_requests = None;
        let mut sec_fetch_dest = None;
        let mut sec_fetch_mode = None;
        let mut sec_fetch_site = None;
        let mut sec_fetch_user = None;

        for line in buffer_iter {
            let line = line.to_string();
            let mut line_iter = line.split(": ");
            let name = line_iter.next().unwrap_or_default();
            let value = line_iter.next().unwrap_or_default();

            match name {
                "Host" => host = Some(String::from(value)),
                "User-Agent" => user_agent = Some(String::from(value)),
                "Accept" => accept = Some(String::from(value)),
                "Accept-Language" => accept_language = Some(String::from(value)),
                "Accept-Encoding" => accept_encoding = Some(String::from(value)),
                "DNT" => dnt = Some(String::from(value)),
                "Connection" => connection = Some(String::from(value)),
                "Upgrade-Insecure-Requests" => upgrade_insecure_requests = Some(String::from(value)),
                "Sec-Fetch-Dest" => sec_fetch_dest = Some(String::from(value)),
                "Sec-Fetch-Mode" => sec_fetch_mode = Some(String::from(value)),
                "Sec-Fetch-Site" => sec_fetch_site = Some(String::from(value)),
                "Sec-Fetch-User" => sec_fetch_user = Some(String::from(value)),
                _ => continue,
            }
        }

        Request { protocol, path, version, host, user_agent, accept, accept_language, accept_encoding, dnt, connection, upgrade_insecure_requests, sec_fetch_dest, sec_fetch_mode, sec_fetch_site, sec_fetch_user }
    }
}

/// HTTP Response
pub struct Response {
    html_path: Option<String>,
    status: Status,
}

impl Response {
    /// Create new response from request
    pub fn from_request(request: Request) -> Result<Self, Box<dyn Error>> {
        let path = request.path.unwrap();
        let mut html_path = String::new();
        let mut status = Status::NotFound;

        if path.ends_with(".ico") {
            html_path.push_str("html/404.html");
        } else if path == "/" {
            html_path.push_str("html/index.html");
            status = Status::Ok;
        } else {
            html_path.push_str("html");
            html_path.push_str(&path);
            html_path.push_str(".html");
            status = Status::Ok;
        }

        println!("HTTPResponse found file {}", html_path);

        Ok(Response { html_path: Some(html_path), status })
    }

    /// Create Error Response that gets send when Request can not be parsed
    pub fn from_error() -> Self {
        Response { html_path: None, status: Status::NotFound }
    }
    
    /// Get response as String
    /// 
    /// # Errors
    /// 
    /// Errors if html text can not be found.
    pub fn text(self) -> Result<String, Box<dyn Error>> {
        let path = match self.status {
            Status::Ok => self.html_path.unwrap(),
            Status::NotFound => String::from("html/404.html"),
        };
        
        println!("HTTPResponse status {:?} send file {}", self.status, path);

        let contents =  fs::read_to_string(path)?;
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            self.status.to_string(), contents.len(), contents
        );

        Ok(response)
    }
}

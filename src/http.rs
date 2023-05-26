use std::{fs, error::Error};

/// HTTP Request
pub struct HTTPRequest {
    buffer: [u8; 1024],
}

impl HTTPRequest {
    /// Create new HTTP Request from buffer
    pub fn new(buffer: [u8; 1024]) -> Self {
        HTTPRequest { buffer }
    }

    /// Get path that was requested
    /// 
    /// # Errors
    /// 
    /// Errors if path can not be extracted.
    pub fn get_path(self) -> Result<String, Box<dyn Error>> {
        let mut iter = self.buffer.split(|c| *c == b' ');
        iter.next();
        let temp = iter.next().unwrap();
        let ret = String::from_utf8(temp.to_vec())?;
        println!("request for path {}", ret);

        Ok(ret)
    }
}

/// HTTP Response Status
enum HTTPResponseStatus {
    Ok,
    NotFound,
}

impl HTTPResponseStatus {
    /// Get Response Status as String
    pub fn text(self) -> String {
        match self {
            HTTPResponseStatus::Ok => "HTTP/1.1 200 OK",
            HTTPResponseStatus::NotFound => "HTTP/1.1 404 NOT FOUND",
        }.to_string()
    }
}

/// HTTP Response
pub struct HTTPResponse {
    html_path: Option<String>,
    status: HTTPResponseStatus,
}

impl HTTPResponse {
    /// Create new response from request
    pub fn new(request: HTTPRequest) -> Result<Self, Box<dyn Error>> {
        let path = request.get_path()?;

        let (html_path, status) = match &path[..] {
            "/" => ("html/hello.html", HTTPResponseStatus::Ok),
            // "/favicon.ico" => "ico/favivon.ico",
            _ => ("html/404.html", HTTPResponseStatus::NotFound),
        };
        let html_path = html_path.to_string();

        Ok(HTTPResponse { html_path: Some(html_path), status })
    }

    pub fn from_error() -> Self {
        HTTPResponse { html_path: None, status: HTTPResponseStatus::NotFound }
    }

    /// Get response as String
    /// 
    /// # Errors
    /// 
    /// Errors if html text can not be found.
    pub fn text(self) -> Result<String, Box<dyn Error>> {
        let contents = fs::read_to_string(self.html_path.unwrap())?;
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            self.status.text(), contents.len(), contents
        );

        Ok(response)
    }
}

/// HTTP Response Status
#[derive(Debug)]
pub enum Status {
    Ok,
    NotFound,
}

impl Status {
    /// Get Response Status as String
    pub fn to_string(self) -> String {
        match self {
            Status::Ok => "HTTP/1.1 200 OK",
            Status::NotFound => "HTTP/1.1 404 NOT FOUND",
        }.to_string()
    }
}

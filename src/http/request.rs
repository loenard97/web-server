#[derive(Debug)]
pub enum Protocol {
    Get,
    Invalid,
}

impl Protocol {
    pub fn from_str(protocol: &str) -> Self {
        match protocol {
            "GET" => Self::Get,
            _ => Self::Invalid, 
        }
    }
}

#[derive(Debug)]
pub enum Version {
    Http1_1,
    Invalid,
}

impl Version {
    pub fn from_str(version: &str) -> Self {
        match version {
            "HTTP/1.1" => Self::Http1_1,
            _ => Self::Invalid, 
        }
    }
}

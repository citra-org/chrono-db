use std::io::{Error, ErrorKind};

pub mod codes {


}

pub enum ErrorCode {
    E123,
    E12343,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::E123 => "e123",
            ErrorCode::E12343 => "e12343",
        }
    }
}

impl From<&str> for ErrorCode {
    fn from(code: &str) -> Self {
        match code {
            "e123" => ErrorCode::E123,
            "e12343" => ErrorCode::E12343,
            _ => panic!("Invalid error code"),
        }
    }
}


// use std::io::{Error, ErrorKind};

pub mod codes {}

pub enum ErrorCode {
    E404,
    E123,
    E12343,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::E404 => "e404",
            ErrorCode::E123 => "e123",
            ErrorCode::E12343 => "e12343",
        }
    }
}

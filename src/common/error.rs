use std::fmt;

#[derive(Debug, Clone)]
pub enum HwpError {
    InvalidFormat(String),
    InvalidSignature,
    UnsupportedVersion(String),
    ParseError(String),
    IoError(String),
    ZipError(String),
    InvalidData(String),
    NotFound(String),
}

impl fmt::Display for HwpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HwpError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            HwpError::InvalidSignature => write!(f, "Invalid file signature"),
            HwpError::UnsupportedVersion(msg) => write!(f, "Unsupported version: {}", msg),
            HwpError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            HwpError::IoError(msg) => write!(f, "IO error: {}", msg),
            HwpError::ZipError(msg) => write!(f, "Zip error: {}", msg),
            HwpError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            HwpError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl From<String> for HwpError {
    fn from(msg: String) -> Self {
        HwpError::ParseError(msg)
    }
}

impl From<&str> for HwpError {
    fn from(msg: &str) -> Self {
        HwpError::ParseError(msg.to_string())
    }
}

pub type HwpResult<T> = Result<T, HwpError>;

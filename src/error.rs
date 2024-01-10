use std::error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    // Define other error types as needed
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO error: {}", e),
            // Handle other errors
        }
    }
}

impl error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

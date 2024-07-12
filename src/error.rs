use std::fmt::Display;

#[derive(Debug)]
pub struct AppError {
    message: String,
}

impl AppError {
    pub fn new(err: impl std::error::Error) -> AppError {
        Self {
            message: err.to_string(),
        }
    }
    pub fn from_string(err_msg: &str) -> AppError {
        Self {
            message: err_msg.to_string(),
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

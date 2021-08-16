use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct ChadError {
    message: String,
}

impl<T: Error> From<T> for ChadError {
    fn from(error: T) -> ChadError {
        ChadError { message: format!("{}", error) }
    }
}


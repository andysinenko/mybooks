use std::error::Error;
use std::fmt::{Display, Formatter};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BooksError {
    pub message: String
}

impl Display for BooksError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for BooksError {

}
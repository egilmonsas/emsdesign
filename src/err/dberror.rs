use std::fmt;

use polars::prelude::PolarsError;

#[derive(Debug)]
pub struct DbError {
    pub message: String,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for OurError {}

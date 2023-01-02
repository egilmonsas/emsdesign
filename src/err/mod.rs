use std::error::Error;
use std::fmt;

use serde::Serialize;

#[allow(unused)]
#[derive(Serialize)]
pub struct EmsError {
    error_kind: EmsErrorKind,
    message: String,
    #[serde(skip_serializing)]
    debug: Option<Box<dyn Error>>,
}

#[derive(Serialize)]
pub enum EmsErrorKind {
    WriteError,
    FileNotFound,
    FieldNotFound,
    DataBaseErr,
}

impl fmt::Display for EmsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl fmt::Debug for EmsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.message, f)
    }
}

impl Error for EmsError {}

impl EmsError {
    fn new(error_kind: EmsErrorKind, message: String, debug: Option<Box<dyn Error>>) -> Self {
        if debug.is_some() {
            // Future place for implementing logging and such
        }
        Self {
            error_kind,
            message,
            debug,
        }
    }
    #[must_use]
    pub fn write_error(message: String, debug: Option<Box<dyn Error>>) -> Self {
        Self::new(EmsErrorKind::WriteError, message, debug)
    }
    #[must_use]
    pub fn file_not_found_error(message: String, debug: Option<Box<dyn Error>>) -> Self {
        Self::new(EmsErrorKind::FileNotFound, message, debug)
    }
    #[must_use]
    pub fn database_error(message: String, debug: Option<Box<dyn Error>>) -> Self {
        Self::new(EmsErrorKind::DataBaseErr, message, debug)
    }
}

use std::error::Error;
use std::fmt;

#[allow(unused)]
pub struct EmsError {
    error_kind: EmsErrorKind,
    message: String,
    debug: Option<Box<dyn Error>>,
}

pub enum EmsErrorKind {
    WriteError,
    FileNotFound,
    FieldNotFound,
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

    pub fn file_not_found(message: String, debug: Option<Box<dyn Error>>) -> Self {
        Self::new(EmsErrorKind::FileNotFound, message, debug)
    }
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NotVipError {
    message: String,
    source: Option<Box<dyn Error + 'static>>,
}

impl NotVipError {
    pub fn new(message: String, source: Option<Box<dyn Error + 'static>>) -> NotVipError {
        NotVipError { message, source }
    }
}

impl fmt::Display for NotVipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for NotVipError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| &**e)
    }
}

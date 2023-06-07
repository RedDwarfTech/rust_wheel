
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NotVipError;

impl fmt::Display for NotVipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vip-expired")
    }
}

impl Error for NotVipError {}
use std::fmt;
use crate::model::error::user::user_info_not_match::UserInfoNotMatchError;

#[derive(Debug)]
pub enum CustomError {
    HttpError,
    ParseError,
    UserInfoNotMatchError,
}

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::HttpError => write!(f, "HTTP Error"),
            CustomError::ParseError => write!(f, "Parse Error"),
            CustomError::UserInfoNotMatchError => write!(f, "UserInfoNotMatch Error"),
        }
    }
}

impl From<UserInfoNotMatchError> for CustomError{
    fn from(_: UserInfoNotMatchError) -> Self {
        CustomError::UserInfoNotMatchError
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(_: reqwest::Error) -> Self {
        CustomError::HttpError
    }
}

impl From<chrono::format::ParseError> for CustomError {
    fn from(_: chrono::format::ParseError) -> Self {
        CustomError::ParseError
    }
}


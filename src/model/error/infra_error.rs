use thiserror::Error;

use super::error_response::ErrorResponse;

#[derive(Error, Debug)]
pub enum InfraError {
    #[error("新旧密码不能相同")]
    NewOldPwdDuplicate,
    #[error("密码不够安全,密码必须包含大写、小写、数字和特殊字符，且长度是8-32位")]
    PwdNitMatchComplexGuide,
}

impl ErrorResponse for InfraError {
    fn error_code(&self) -> &str {
        match self {
            InfraError::NewOldPwdDuplicate => "0030010008",
            InfraError::PwdNitMatchComplexGuide => "0030010006",
        }
    }

    fn error_message(&self) -> &str {
        match self {
            InfraError::NewOldPwdDuplicate => "新旧密码不能相同",
            InfraError::PwdNitMatchComplexGuide => {
                "密码不够安全,密码必须包含大写、小写、数字和特殊字符，且长度是8-32"
            }
        }
    }

    fn error_code_en(&self) -> &str {
        match self {
            InfraError::NewOldPwdDuplicate => "NEW_OLD_PWD_DUPLICATED",
            InfraError::PwdNitMatchComplexGuide => "PWD_NOT_MATCH_COMPLAEX_GUIDE",
        }
    }
}

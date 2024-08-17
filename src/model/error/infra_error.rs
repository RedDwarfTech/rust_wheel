use thiserror::Error;

use super::error_response::ErrorResponse;

#[derive(Error, Debug)]
pub enum InfraError {
    #[error("新旧密码不能相同")]
    NewOldPwdDuplicate,
    #[error("密码不够安全,密码必须包含大写、小写、数字和特殊字符，且长度是8-32位")]
    PwdNitMatchComplexGuide,
    #[error("数据未找到")]
    DataNotFound,
    #[error("短信验证码不匹配")]
    SmsVerifyCodeNotMatch,
    #[error("登录信息不匹配")]
    LoginInfoNotMatch,
    #[error("验证码已过期")]
    VerifyCodeExpired,
    #[error("无访问资源权限")]
    AccessResourceDenied,
}

impl ErrorResponse for InfraError {
    fn error_code(&self) -> &str {
        match self {
            InfraError::NewOldPwdDuplicate => "0030010008",
            InfraError::PwdNitMatchComplexGuide => "0030010006",
            InfraError::DataNotFound => "0030010009",
            InfraError::SmsVerifyCodeNotMatch => "0030010010",
            InfraError::LoginInfoNotMatch => "0030010001",
            InfraError::VerifyCodeExpired => "0030010011",
            InfraError::AccessResourceDenied => "0030010012",
        }
    }

    fn error_message(&self) -> &str {
        match self {
            InfraError::NewOldPwdDuplicate => "新旧密码不能相同",
            InfraError::PwdNitMatchComplexGuide => {
                "密码不够安全,密码必须包含大写、小写、数字和特殊字符，且长度是8-32"
            },
            InfraError::DataNotFound => {
                "数据未找到"
            },
            InfraError::SmsVerifyCodeNotMatch => {
                "短信验证码不匹配"
            },
            InfraError::LoginInfoNotMatch => {
                "登录信息不匹配"
            },
            InfraError::VerifyCodeExpired => {
                "验证码已过期"
            },
            InfraError::AccessResourceDenied => {
                "无访问资源权限"
            },
        }
    }

    fn error_code_en(&self) -> &str {
        match self {
            InfraError::NewOldPwdDuplicate => "NEW_OLD_PWD_DUPLICATED",
            InfraError::PwdNitMatchComplexGuide => "PWD_NOT_MATCH_COMPLAEX_GUIDE",
            InfraError::DataNotFound => "DATA_NOT_FOUND",
            InfraError::SmsVerifyCodeNotMatch => "SMS_VERIFY_CODE_NOT_MATCH",
            InfraError::LoginInfoNotMatch => "LOGIN_INFO_NOT_MATCH",
            InfraError::VerifyCodeExpired => "VERIFY_CODE_EXPIRED",
            InfraError::AccessResourceDenied => "ACCESS_RESOURCE_DENIED",
        }
    }
}

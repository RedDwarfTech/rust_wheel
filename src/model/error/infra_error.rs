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
    #[error("签名已过期")]
    SignExpired,
    #[error("签名非法")]
    SignIlleagal,
    #[error("超出文件数量限制")]
    FileNumLimitExceed,
    #[error("登录错误次数过多")]
    LoginErrorTooMany,
    #[error("用户已注册")]
    UserAlreadyRegistered,
    #[error("APPID不匹配")]
    AppIdNotMatch,
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
            InfraError::SignExpired => "0030010013",
            InfraError::SignIlleagal => "0030010014",
            InfraError::FileNumLimitExceed => "001002D001",
            InfraError::LoginErrorTooMany => "0030010002",
            InfraError::UserAlreadyRegistered => "0030010005",
            InfraError::AppIdNotMatch => "0030010007",
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
            InfraError::SignExpired => {
                "签名已过期"
            },
            InfraError::SignIlleagal => {
                "签名非法"
            },
            InfraError::FileNumLimitExceed => {
                "超出文件数量限制"
            },
            InfraError::LoginErrorTooMany => {
                "登录错误次数过多"
            },
            InfraError::UserAlreadyRegistered => {
                "用户已注册"
            },
            InfraError::AppIdNotMatch => {
                "APPID不匹配"
            }
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
            InfraError::SignExpired => "SIGN_EXPIRED",
            InfraError::SignIlleagal => "SIGN_ILLEAGAL",
            InfraError::FileNumLimitExceed => "FILE_NUM_LIMIT_EXCEED",
            InfraError::LoginErrorTooMany => "LOGIN_FAILED_TOO_MUCH",
            InfraError::UserAlreadyRegistered => "USER_ALREADY_REGISTERED",
            InfraError::AppIdNotMatch => "APPID_NOT_MATCH",
        }
    }
}

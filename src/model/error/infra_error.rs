use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfraError {
    #[error("新旧密码不能相同")]
    NewOldPwdDuplicate,
}
impl InfraError {
    pub fn error_code(&self) -> &str {
        match self {
            InfraError::NewOldPwdDuplicate => "0030010008",
        }
    }

    pub fn error_message(&self) -> &str {
        match self {
            InfraError::NewOldPwdDuplicate => "新旧密码不能相同"            
        }
    }

    pub fn error_code_en(&self) -> &str {
        match self {
            InfraError::NewOldPwdDuplicate => "NEW_OLD_PWD_DUPLICATED"
        }
    }
}

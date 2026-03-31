use std::future::Future;

use tokio::task_local;

use crate::model::user::login_user_info::LoginUserInfo;

task_local! {
    static CURRENT_USER: LoginUserInfo;
}

pub struct ContextUtil;

impl ContextUtil {

    pub async fn with_user<F, T>(user: LoginUserInfo, f: F) -> T 
    where
        F: Future<Output = T>,
    {
        CURRENT_USER.scope(user, f).await
    }
    
    pub fn current_user() -> Result<LoginUserInfo, &'static str> {
        // 注意：必须在 task_local! 作用域内调用
        CURRENT_USER.try_with(|user| user.clone())
            .map_err(|_| "User context not found")
    }
}
use std::future::Future;

use log::error;
use log::info;
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
        info!("set user to context: {}", serde_json::to_string(&user).unwrap());
        CURRENT_USER.scope(user, f).await
    }
    
    pub fn current_user() -> Result<LoginUserInfo, &'static str> {
        match CURRENT_USER.try_with(|user| user.clone()) {
            Ok(user) => Ok(user),
            Err(err) => {
                error!("ContextUtil::current_user failed: user context not found, err={:?}", err);
                Err("User context not found")
            }
        }
    }
}
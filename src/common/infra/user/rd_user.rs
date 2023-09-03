use log::error;
use reqwest::Client;
use crate::model::{user::rd_user_info::RdUserInfo, response::api_response::ApiResponse};

pub async fn get_user_info(input_user_id: &i64) -> Option<RdUserInfo> {
    let client = Client::new();
    let url = "http://dolphin-post-service.reddwarf-pro.svc.cluster.local:11014/post/user/";
    let resp = client.get(format!("{}{}", url, input_user_id))
    .body("{}")
    .send()
        .await;
    if let Err(e) = resp {
        error!("get user failed: {}", e);
        return None;
    }
    let text_response = resp.unwrap().text().await;
    if let Err(e) = text_response {
        error!("extract text failed: {}", e);
        return None;
    }
    let resp_result = serde_json::from_str::<ApiResponse<RdUserInfo>>(&text_response.unwrap());
    if let Err(pe) = resp_result {
        error!("parse failed: {}", pe);
        return None;
    }
    Some(resp_result.unwrap().result)
}
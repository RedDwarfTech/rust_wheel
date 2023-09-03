pub async fn get_user_info(input_user_id: i64) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = "http://dolphin-post-service.reddwarf-pro.svc.cluster.local:11014/post/user/";
    let uid = string_to_static_str(input_user_id.to_string());
    let resp = reqwest::get(format!("{}{}", url, uid))
        .await?
        .json::<serde_json::Value>()
        .await?;
    Ok(resp)
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
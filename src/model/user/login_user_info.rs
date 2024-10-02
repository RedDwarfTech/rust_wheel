use rocket::{Request, request};
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use serde_json::{from_str, Value};

// https://stackoverflow.com/questions/24102325/warning-function-should-have-a-snake-case-identifier-on-by-default
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, Clone)]
#[allow(non_snake_case)]
pub struct LoginUserInfo {
    pub token: String,
    pub userId: i64,
    pub appId: String,
    pub xRequestId: String,
    pub deviceId: String,
    pub vipExpireTime: i64,
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

fn get_auth_from_header(req: &Request<'_>) -> String {
    let token = req.headers().get_one("Authorization");
    if token.unwrap().starts_with("Bearer ") {
        let token = token.unwrap().trim_start_matches("Bearer ");
        return token.to_string();
    }
    return "".to_string();
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoginUserInfo {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // https://github.com/halfrost/Halfrost-Field/blob/master/contents/Protocol/HTTP:2-HTTP-Semantics.md#2-http-header-fields
        // HTTP 头字段通过一系列的键值对的方式来携带信息。有关已注册 HTTP headers 的列表，
        // 请参阅 https://www.iana.org/assignments/message-headers 上维护的“消息头字段”已注册列表。
        // 与 HTTP/1.x 中一样， header 字段名称是 ASCII 字符串，以不区分大小写的方式进行比较。
        // 但是，在 HTTP/2 编码之前，必须将 Header 头字段名称转换为小写。
        // 包含大写 header 字段名称的请求或响应必须被视为格式错误（第 8.1.2.6 节）。
        let token = get_auth_from_header(request);
        let x_request_id = request.headers().get_one("x-request-id");
        let parts: Vec<&str> = token.split(".").collect();
        let payload_base64 = parts[1];
        let payload_str = base64::decode(payload_base64).unwrap();
        let payload_json = from_str::<serde_json::Value>(&String::from_utf8(payload_str).unwrap()).unwrap();
        let payload_claims = payload_json.as_object().unwrap();
        let user_id = payload_claims.get("userId");
        let app_id = payload_claims.get("appId");
        let device_id = payload_claims.get("deviceId");
        let vip_expire_time:Option<&Value> = payload_claims.get("et");
        let login_user_info = LoginUserInfo {
            token: token.to_string(),
            userId: user_id.unwrap().as_i64().unwrap(),
            appId: app_id.unwrap().to_string(),
            xRequestId: x_request_id.unwrap().to_string(),
            deviceId: device_id.unwrap().to_string(),
            vipExpireTime: vip_expire_time.unwrap_or(&Value::Null).as_i64().unwrap_or_default()
        };
        Outcome::Success(login_user_info)
         
    }
}

/**
Q: My (diesel) database does not implement OpenApiFromRequest.
A: This is because the parameter does not show up in the path, query or body.
So this is considered a Request Guard. There is a derive macro for this,
but this does not work in combination with the #[database("...")] marco.
You can solve this my implementing it manually
**/
impl<'r> OpenApiFromRequest<'r> for LoginUserInfo {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
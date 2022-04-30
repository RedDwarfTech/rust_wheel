use rocket::{Request, request};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;

// https://stackoverflow.com/questions/24102325/warning-function-should-have-a-snake-case-identifier-on-by-default
#[allow(non_snake_case)]
pub struct LoginUserInfo {
    pub token: String,
    pub userId: i64,
    pub appId: String,
    pub xRequestId: String
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
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
        let token = request.headers().get_one("x-access-token");
        let app_id = request.headers().get_one("app-id");
        let user_id = request.headers().get_one("user-id");
        let x_request_id = request.headers().get_one("x-request-id");
        match token {
            Some(token) => {
                let login_user_info = LoginUserInfo {
                    token: token.to_string(),
                    userId: user_id.unwrap().parse::<i64>().unwrap(),
                    appId: app_id.unwrap().to_string(),
                    xRequestId: x_request_id.unwrap().to_string()
                };
                // check validity
                Outcome::Success(login_user_info)
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}
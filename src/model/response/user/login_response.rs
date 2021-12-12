use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct LoginResponse {
    pub registerTime: i64,
    pub refreshToken: String,
    pub accessToken: String
}

impl Default for LoginResponse {
    fn default() -> Self {
        LoginResponse {
            registerTime: 0,
            refreshToken: "".to_string(),
            accessToken: "".to_string()
        }
    }
}

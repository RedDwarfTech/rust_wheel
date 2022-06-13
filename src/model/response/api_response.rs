use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::{Request, Response};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, JsonSchema, Serialize)]
#[allow(non_snake_case)]
pub struct ApiResponse<T> {
    pub result: T,
    pub statusCode: String,
    pub resultCode: String,
    pub msg: String
}

impl<'r, T> Responder<'r, 'r> for ApiResponse<T> {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        let response = Response::new();
        Response::build_from(response)
            .header(ContentType::JSON)
            .ok()
    }
}

impl<T> Default for ApiResponse<T>
where
    T: Default,
{
    fn default() -> Self {
        ApiResponse {
            result: T::default(),
            statusCode: "200".to_string(),
            resultCode: "200".to_string(),
            msg: "ok".to_string()
        }
    }
}

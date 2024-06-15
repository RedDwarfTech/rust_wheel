pub trait ErrorResponse {
    fn error_code_en(&self) -> &str;
    fn error_code(&self) -> &str;
    fn error_message(&self) -> &str;
}
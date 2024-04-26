pub enum JwtTokenError {
    Valid,
    Invalid,
    Expired,
    OtherError,
}
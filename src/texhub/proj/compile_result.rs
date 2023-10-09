#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum CompileResult {
    Success = 0,
    Failure = 1,
    Unknown = -1
}
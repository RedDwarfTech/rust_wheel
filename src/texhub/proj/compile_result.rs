#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum CompileResult {
    Success = 0,
    Failure = 1,
    Unknown = -1
}

impl From<CompileResult> for i32 {
    fn from(result: CompileResult) -> Self {
        match result {
            CompileResult::Success => 0,
            CompileResult::Failure => 1,
            CompileResult::Unknown => -1,
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum CompileStatus {
    Queued = 0,
    Compiling = 1,
    Complete = 2,
}
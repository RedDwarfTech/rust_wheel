#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct UserInfoNotMatchError(NotMatchKind);

/// The category of parse error
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum NotMatchKind {
    /// Given field is out of permitted range.
    Mismatch,
}
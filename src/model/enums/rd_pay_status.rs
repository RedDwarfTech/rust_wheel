#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum RdPayStatus {
    Success = 1,
    Failed = 2
}

impl From<RdPayStatus> for i32 {
    fn from(online_status: RdPayStatus) -> Self {
        match online_status {
            RdPayStatus::Success => 1,
            RdPayStatus::Failed => 2,
        }
    }
}
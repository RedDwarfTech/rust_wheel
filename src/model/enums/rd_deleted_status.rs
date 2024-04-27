#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum RdDeletedStatus {
    Normal = 0,
    Deleted = 1,
}

impl From<RdDeletedStatus> for i32 {
    fn from(online_status: RdDeletedStatus) -> Self {
        match online_status {
            RdDeletedStatus::Normal => 0,
            RdDeletedStatus::Deleted => 1,
        }
    }
}
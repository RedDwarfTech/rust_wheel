#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum RdOnlineStatus {
    Offline = 0,
    Online = 1,
}

impl From<RdOnlineStatus> for i32 {
    fn from(online_status: RdOnlineStatus) -> Self {
        match online_status {
            RdOnlineStatus::Online => 0,
            RdOnlineStatus::Offline => 1,
        }
    }
}
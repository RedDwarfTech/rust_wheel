#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum RdAccountType {
    Wechat = 1,
    Weibo = 2,
    Netease = 3,
    Guest = 4,
}

impl From<RdAccountType> for i32 {
    fn from(online_status: RdAccountType) -> Self {
        match online_status {
            RdAccountType::Wechat => 1,
            RdAccountType::Weibo => 2,
            RdAccountType::Netease => 3,
            RdAccountType::Guest => 4,
        }
    }
}
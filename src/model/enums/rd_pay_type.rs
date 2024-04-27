#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum RdPayType {
    Alipay = 5
}

impl From<RdPayType> for i32 {
    fn from(online_status: RdPayType) -> Self {
        match online_status {
            RdPayType::Alipay => 5,
        }
    }
}
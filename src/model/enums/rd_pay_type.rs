#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum RdPayType {
    Alipay = 2,
    Wechat = 1,
    Paypal = 3
}

impl From<RdPayType> for i32 {
    fn from(online_status: RdPayType) -> Self {
        match online_status {
            RdPayType::Alipay => 2,
            RdPayType::Wechat => 1,
            RdPayType::Paypal => 3,
        }
    }
}
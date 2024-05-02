#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum RdOrderStatus {
    WaitingForPayment = 0,
    PAID = 1,
    SHIPPED = 2,
    COMPLETED = 3,
    CANCELED = 4
}

impl From<RdOrderStatus> for i32 {
    fn from(online_status: RdOrderStatus) -> Self {
        match online_status {
            RdOrderStatus::WaitingForPayment => 0,
            RdOrderStatus::PAID => 1,
            RdOrderStatus::SHIPPED => 2,
            RdOrderStatus::COMPLETED => 3,
            RdOrderStatus::CANCELED => 4,
        }
    }
}
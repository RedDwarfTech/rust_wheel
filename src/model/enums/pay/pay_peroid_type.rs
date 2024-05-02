#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum PayPeroidType {
    DAY = 1,
    OneMonth = 2,
    ThreeMonth = 3,
    SixMonth = 4,
    OneYear = 5
}

impl From<PayPeroidType> for i32 {
    fn from(online_status: PayPeroidType) -> Self {
        match online_status {
            PayPeroidType::DAY => 1,
            PayPeroidType::OneMonth => 2,
            PayPeroidType::ThreeMonth => 3,
            PayPeroidType::SixMonth => 4,
            PayPeroidType::OneYear => 5,
        }
    }
}
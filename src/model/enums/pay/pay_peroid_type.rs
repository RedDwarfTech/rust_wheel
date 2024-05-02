#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum PayPeroidType {
    DAY = 1,
    OneMonth = 2,
    ThreeMonth = 3,
    SixMonth = 4,
    OneYear = 5,
    Unknow = 6
}

impl From<PayPeroidType> for i32 {
    fn from(online_status: PayPeroidType) -> Self {
        match online_status {
            PayPeroidType::DAY => 1,
            PayPeroidType::OneMonth => 2,
            PayPeroidType::ThreeMonth => 3,
            PayPeroidType::SixMonth => 4,
            PayPeroidType::OneYear => 5,
            PayPeroidType::Unknow => 6,
        }
    }
}

impl From<i32> for PayPeroidType {
    fn from(online_status: i32) -> Self {
        match online_status {
            1 => PayPeroidType::DAY,
            2 => PayPeroidType::OneMonth,
            3 => PayPeroidType::ThreeMonth,
            4 => PayPeroidType::SixMonth,
            5 => PayPeroidType::OneYear,
            _ => PayPeroidType::Unknow
        }
    }
}
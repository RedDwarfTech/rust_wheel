#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum ApplePayProductType {
    CONSUMABLE = 1,
    NonConsumable = 2,
    SUBSCRIPTION = 3,
    NonSubscription = 4
}

impl From<ApplePayProductType> for i32 {
    fn from(online_status: ApplePayProductType) -> Self {
        match online_status {
            ApplePayProductType::CONSUMABLE => 1,
            ApplePayProductType::NonConsumable => 2,
            ApplePayProductType::SUBSCRIPTION => 3,
            ApplePayProductType::NonSubscription => 4,
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum ChannelStatus {
    Backup = 0,
    Ok = 1,
    Unavaliable = -1,
    LowQuality = -3,
    URLError = -4,
    ParsedFailed = -5,
    UnknownRssType = -6,
}
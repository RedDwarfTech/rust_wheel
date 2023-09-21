#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum ThFileType {
    Folder = 0,
    Tex = 1,
}

impl From<ThFileType> for i32 {
    fn from(file_type: ThFileType) -> Self {
        match file_type {
            ThFileType::Folder => 0,
            ThFileType::Tex => 1,
        }
    }
}
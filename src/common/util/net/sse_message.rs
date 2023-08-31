use uuid::Uuid;

pub struct SSEMessage {
    pub event: Option<String>,
    pub data: String,
    pub id: Option<String>,
    pub retry: Option<u32>,
}

impl From<&String> for SSEMessage {
    fn from(data: &String) -> Self {
        Self {
            event: Some("".to_owned()),
            data: data.to_string(),
            id: Some("".to_owned()),
            retry: None,
        }
    }
}

impl SSEMessage {
    pub fn from_data(data: &String, event_type: &String) -> Self {
        let uuid = Uuid::new_v4();
        let uuid_string = uuid.to_string().replace("-", "");
        Self {
            event: Some(event_type.to_owned()),
            data: data.to_string(),
            id: Some(uuid_string),
            retry: Some(5000),
        }
    }
}

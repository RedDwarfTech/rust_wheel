use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
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

impl fmt::Display for SSEMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(event) = &self.event {
            write!(f, "event:{}\n", event)?;
        }
        write!(f, "data:{}\n", self.data)?;
        if let Some(id) = &self.id {
            write!(f, "id:{}\n", id)?;
        }
        if let Some(retry) = &self.retry {
            write!(f, "retry:{}\n", retry)?;
        }
        write!(f, "\n")
    }
}

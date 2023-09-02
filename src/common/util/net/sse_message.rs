use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Clone)]
pub struct SSEMessage<T> where T: fmt::Display, {
    pub event: Option<String>,
    pub data: T,
    pub id: Option<String>,
    pub retry: Option<u32>,
}

impl<T: fmt::Display> From<&String> for SSEMessage<T> {
    fn from(data: T) -> Self {
        Self {
            event: Some("".to_string()),
            data: data,
            id: Some("".to_string()),
            retry: None,
        }
    }
}

impl<T: fmt::Display> SSEMessage<T> {
    pub fn from_data(data: T, event_type: &String) -> Self {
        Self {
            event: Some(event_type.to_string()),
            data: data,
            id: None,
            retry: Some(5000),
        }
    }
}

impl<T: fmt::Display> fmt::Display for SSEMessage<T> {
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
